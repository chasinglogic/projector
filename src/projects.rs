use std::fs::{metadata, File, OpenOptions};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::time::Duration;

use walkdir;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ignore_cache: bool,
    pub code_dir: String,
    pub cache_file: PathBuf,
    pub ignore_patterns: Vec<String>,
}

impl Config {
    pub fn new(code_dir: String) -> Config {
        Config {
            code_dir: code_dir,
            ignore_cache: false,
            cache_file: PathBuf::from(".projector_cache"),
            ignore_patterns: Vec::new(),
        }
    }

    pub fn ignore_cache(self) -> Config {
        self.ignore_cache = true;
        self
    }

    pub fn ignore_patterns(self, patterns: Vec<String>) -> Config {
        self.ignore_patterns = patterns.clone();
        self
    }

    pub fn cache_file(self, cache_file: PathBuf) -> Config {
        self.cache_file = cache_file;
        self
    }
}

fn ignore_error(e: walkdir::Error) -> bool {
    if let Some(path) = e.path() {
        if path.is_file() {
            return true;
        }
    }

    if let Some(err) = e.io_error() {
        if err.kind() == ErrorKind::NotFound {
            return true;
        }
    }

    return false;
}

fn should_use_cache(cache_file: &Path) -> bool {
    // Obviously if there's no cache we shouldn't use it.
    if !cache_file.exists() {
        return false;
    }

    let cache_file_md = metadata(cache_file);

    match cache_file_md {
        Ok(md) => {
            let last_cache_access;
            if let Ok(time) = md.modified() {
                last_cache_access = time.elapsed().unwrap_or(Duration::new(0, 0));
                // 172800 is 48 hours in seconds. Rust doesn't have a better
                // way to work with time.
                return last_cache_access.as_secs() < 172800;
            }

            true
        }
        Err(_) => false,
    }
}

pub fn find_projects<F>(config: Config, callback: F)
where
    F: Fn(&str) -> (),
{
    let use_cache = !config.ignore_cache && should_use_cache(&config.cache_file);

    if use_cache {
        find_projects_from_cache(config, callback);
    } else {
        find_projects_from_fs(config, callback);
    }
}

fn find_projects_from_cache<F>(config: Config, callback: F)
where
    F: Fn(&str) -> (),
{
    let mut f = File::open(&config.cache_file).expect("Cache file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Unable to read from cache file.");

    for p in contents.as_str().split("\n") {
        callback(p);
    }
}

fn find_projects_from_fs<F>(config: Config, callback: F)
where
    F: Fn(&str) -> (),
{
    let wkd = WalkDir::new(config.code_dir);
    let code_dirs = wkd.into_iter();
    // Holds the projects we find
    let mut projects = Vec::new();

    for dir in code_dirs {
        let cwd;
        match dir {
            Ok(d) => cwd = d,
            Err(e) => {
                if ignore_error(e) {
                    continue;
                } else {
                    println!("Unkown Error: {}", e);
                    process::exit(1);
                }
            }
        };

        if !cwd.file_type().is_dir() {
            continue;
        }

        let file_name = cwd.file_name().to_str().unwrap_or("");
        if file_name == ".git" {
            let parent_path = cwd.path().parent().unwrap().to_str().unwrap_or("");

            callback(&parent_path);
            projects.push(parent_path.to_string().clone());
        }
    }

    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&config.cache_file)
        .expect("Unable to open cache file");

    let results = projects.join("\n");
    match f.write_all(&results.into_bytes()) {
        Ok(_) => {}
        Err(e) => println!("ERROR: Unable to write cache file {}", e),
    }
}
