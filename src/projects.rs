// use rayon::prelude::*;
use regex::Regex;
use std::io::ErrorKind;
use std::process;
use walkdir;
use walkdir::WalkDir;

fn skippable_err(e: &walkdir::Error) -> bool {
    if let Some(path) = e.path() {
        if path.is_file() {
            return true;
        }
    }

    if let Some(err) = e.io_error() {
        match err.kind() {
            ErrorKind::NotFound => return true,
            ErrorKind::PermissionDenied => return true,
            _ => return false,
        }
    }

    println!("ERROR: {}", e);
    return false;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub code_dirs: Vec<String>,
    pub excludes: Option<Vec<String>>,
    pub includes: Option<Vec<String>>,
}

impl Config {
    pub fn new(code_dirs: Vec<String>) -> Config {
        Config {
            code_dirs: code_dirs,
            excludes: None,
            includes: None,
        }
    }

    pub fn one(code_dir: String) -> Config {
        Config::new(vec![code_dir])
    }

    pub fn with_includes(mut self, include_patterns: Vec<String>) -> Config {
        self.includes = Some(include_patterns);
        self
    }

    pub fn with_excludes(mut self, exclude_patterns: Vec<String>) -> Config {
        self.excludes = Some(exclude_patterns);
        self
    }
}

pub fn find<F>(config: Config, callback: F)
where
    F: Fn(String) -> (),
{
    let mut pat;
    let exclude;
    let include;

    if let Some(ref patterns) = config.excludes {
    } else {
        exclude = Regex::new("^$").unwrap();
    };

    if let Some(ref patterns) = config.includes {
        pat = "(".to_string();

        for pattern in patterns {
            pat.push_str(&pattern);
            pat.push_str("|");
        }

        // Remove trailing |
        pat.pop();
        pat.push_str(")");

        include = match Regex::new(&pat) {
            Ok(r) => r,
            Err(e) => {
                println!("ERROR: Unable to compile regex: {}: {}", pat, e);
                process::exit(1);
            }
        }
    } else {
        include = Regex::new("^$").unwrap();
    };

    config.code_dirs.into_iter().for_each(|code_dir| {
        let mut wkd = WalkDir::new(code_dir).into_iter();
        loop {
            let dir = match wkd.next() {
                None => break,
                Some(Ok(dir)) => dir,
                Some(Err(e)) => {
                    if skippable_err(&e) {
                        continue;
                    } else {
                        panic!("ERROR: {}", e);
                    }
                }
            };

            let mut path = dir.path().to_path_buf();
            path.push(".git");

            if path.exists() {
                wkd.skip_current_dir();

                let project_path = dir.path().to_str().unwrap_or("").to_string();
                if exclude.is_match(&project_path) && !include.is_match(&project_path) {
                    continue;
                }

                callback(project_path);
            }
        }
    })
}
