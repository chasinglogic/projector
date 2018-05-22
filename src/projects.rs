use std::env;
use std::fs::{metadata, File, OpenOptions};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::process;
use std::time::Duration;

use walkdir;
use walkdir::WalkDir;

struct Config {
    ignore_cache: bool,
    code_dir: String,
    cache_file: PathBuf,
    ignore_patterns: Vec<String>,
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

    println!("ERROR: {}", e);
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
        find_projects_from_fs(config, &cache_file, callback);
    }
}

fn find_projects_from_cache<F>(cache_file: &Path, callback: F)
where
    F: Fn(&str) -> (),
{
    let mut f = File::open(cache_file).expect("Cache file not found");
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
