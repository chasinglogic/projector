use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::{self, Command};

use super::filter::Filters;
use regex::RegexSet;
use serde::Deserialize;
use walkdir::WalkDir;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub code_dirs: Vec<String>,
    pub excludes: Option<Vec<String>>,
    pub includes: Option<Vec<String>>,
}

impl Config {
    pub fn new(code_dirs: Vec<String>) -> Config {
        Config {
            code_dirs,
            excludes: None,
            includes: None,
        }
    }

    pub fn with_excludes(mut self, excludes: Vec<String>) -> Config {
        self.excludes = Some(excludes);
        self
    }

    pub fn with_includes(mut self, includes: Vec<String>) -> Config {
        self.includes = Some(includes);
        self
    }
}

impl From<&[String]> for Config {
    fn from(dirs: &[String]) -> Config {
        Config::new(dirs.to_vec())
    }
}

impl From<String> for Config {
    fn from(s: String) -> Config {
        Config::new(vec![s])
    }
}

impl From<&str> for Config {
    fn from(s: &str) -> Config {
        Config::from(s.to_string())
    }
}

pub struct Finder {
    code_dirs: Box<dyn Iterator<Item = PathBuf>>,
    walker: walkdir::IntoIter,
    filters: Filters,
    dirty_only: bool,
}

impl Iterator for Finder {
    type Item = PathBuf;

    fn next(&mut self) -> Option<PathBuf> {
        loop {
            let dir = match self.walker.next() {
                Some(Ok(dir)) => dir,
                None => match self.code_dirs.next() {
                    Some(dir) => {
                        self.walker = WalkDir::new(dir).into_iter();
                        continue;
                    }
                    None => return None,
                },
                Some(Err(e)) => {
                    if Finder::skippable_err(&e) {
                        continue;
                    } else {
                        panic!("ERROR: {}", e);
                    }
                }
            };

            let mut path = dir.path().to_path_buf();
            path.push(".git");

            if path.exists() {
                path.pop();
                self.walker.skip_current_dir();

                if self.dirty_only {
                    let mut child = Command::new("git");
                    child.args(["status", "--porcelain"]);

                    let proc = child
                        .current_dir(path.clone())
                        .output()
                        .expect("failed to start git!");

                    if proc.stdout == "".as_bytes() {
                        continue;
                    }
                }

                return Some(path);
            }
        }
    }
}

impl Finder {
    pub fn new(code_dirs: Vec<PathBuf>) -> Finder {
        let dirs = if code_dirs.is_empty() {
            vec![PathBuf::from(".")]
        } else {
            code_dirs
        };

        let filters = Filters::new();

        let mut iter = Box::new(dirs.into_iter());
        Finder {
            walker: WalkDir::new(iter.next().unwrap()).into_iter(),
            code_dirs: iter,
            filters,
            dirty_only: false,
        }
    }

    pub fn with_excludes(mut self, excludes: RegexSet) -> Finder {
        self.filters = self.filters.with_excludes(excludes);
        self
    }

    pub fn with_includes(mut self, includes: RegexSet) -> Finder {
        self.filters = self.filters.with_includes(includes);
        self
    }

    pub fn with_dirty_only(mut self, only_dirty: bool) -> Finder {
        self.dirty_only = only_dirty;
        self
    }

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
                _ => (),
            }
        }

        println!("ERROR: {}", e);
        false
    }
}

// Combine multiple regex strings into a single regex that logical
// "or"s the given regex patterns together.
fn regex_from_patterns(patterns: Vec<String>) -> RegexSet {
    match RegexSet::new(&patterns) {
        Ok(r) => r,
        Err(e) => {
            println!(
                "ERROR: Unable to compile regex: {}: {}",
                patterns.join(" "),
                e
            );
            process::exit(1);
        }
    }
}

impl From<Config> for Finder {
    fn from(cfg: Config) -> Finder {
        Finder::from(cfg.code_dirs)
            .with_excludes(regex_from_patterns(cfg.excludes.unwrap_or_default()))
            .with_includes(regex_from_patterns(cfg.includes.unwrap_or_default()))
    }
}

impl From<&str> for Finder {
    fn from(dir: &str) -> Finder {
        Finder::from(dir.to_string())
    }
}

impl From<String> for Finder {
    fn from(dir: String) -> Finder {
        Finder::new(vec![PathBuf::from(dir)])
    }
}

impl From<Vec<String>> for Finder {
    fn from(dirs: Vec<String>) -> Finder {
        Finder::new(dirs.iter().map(PathBuf::from).collect())
    }
}
