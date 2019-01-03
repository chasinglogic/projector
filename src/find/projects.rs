use std::io::ErrorKind;
use std::path::PathBuf;
use std::process;

use regex::Regex;

use walkdir::WalkDir;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

    pub fn finder(self) -> Finder {
        Finder::from(self)
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
    code_dirs: Box<Iterator<Item = PathBuf>>,
    walker: walkdir::IntoIter,
    excludes: Option<Regex>,
    includes: Regex,
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

                if let Some(ref excludes) = self.excludes {
                    // convert to string for regex matching
                    let s = path.to_str().unwrap_or_default().to_string();
                    if excludes.is_match(&s) && !self.includes.is_match(&s) {
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

        let mut iter = Box::new(dirs.into_iter());
        Finder {
            walker: WalkDir::new(iter.next().unwrap()).into_iter(),
            code_dirs: iter,
            excludes: None,
            // guaranteed to compile to safe so unwrap
            includes: Regex::new("^$").unwrap(),
        }
    }

    pub fn with_excludes(mut self, excludes: Regex) -> Finder {
        self.excludes = Some(excludes);
        self
    }

    pub fn with_includes(mut self, includes: Regex) -> Finder {
        self.includes = includes;
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
                _ => return false,
            }
        }

        println!("ERROR: {}", e);
        false
    }
}

// Combine multiple regex strings into a single regex that logical
// "or"s the given regex patterns together.
fn regex_from_patterns(patterns: Vec<String>) -> Regex {
    let mut pat = "(".to_string();

    for pattern in patterns {
        pat.push_str(&pattern);
        pat.push_str("|");
    }

    // Remove trailing |
    pat.pop();
    pat.push_str(")");

    match Regex::new(&pat) {
        Ok(r) => r,
        Err(e) => {
            println!("ERROR: Unable to compile regex: {}: {}", pat, e);
            process::exit(1);
        }
    }
}

impl From<Config> for Finder {
    fn from(cfg: Config) -> Finder {
        let mut finder = Finder::from(cfg.code_dirs);

        if let Some(patterns) = cfg.excludes {
            finder = finder.with_excludes(regex_from_patterns(patterns));
        }

        if let Some(patterns) = cfg.includes {
            finder = finder.with_includes(regex_from_patterns(patterns));
        }

        finder
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
