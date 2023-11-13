use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::{self, Command};

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
    excludes: RegexSet,
    includes: RegexSet,
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
                        self.walker = WalkDir::new(dir).min_depth(1).into_iter();
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
            let path_string = path.to_str().unwrap_or_default();
            if self.excludes.is_match(path_string) && !self.includes.is_match(path_string) {
                self.walker.skip_current_dir();
                continue;
            }

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

        let mut iter = Box::new(dirs.into_iter());
        Finder {
            walker: WalkDir::new(iter.next().unwrap()).min_depth(1).into_iter(),
            code_dirs: iter,
            // guaranteed to compile to safe so unwrap
            excludes: RegexSet::new(&["^$"]).unwrap(),
            includes: RegexSet::new(&["^$"]).unwrap(),
            dirty_only: false,
        }
    }

    pub fn with_excludes(mut self, excludes: RegexSet) -> Finder {
        self.excludes = excludes;
        self
    }

    pub fn with_includes(mut self, includes: RegexSet) -> Finder {
        self.includes = includes;
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
                _ => return false,
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

#[cfg(test)]
mod test {
    use super::{Config, Finder};
    use std::path::PathBuf;

    macro_rules! create_dirs {
        ($code_dir:expr, $dirs:expr) => {{
            std::fs::remove_dir_all($code_dir).ok();

            let dirs: Vec<PathBuf> = $dirs
                .iter()
                .map(|s| format!("{}/{}", $code_dir, s))
                .map(PathBuf::from)
                .collect();

            for dir in &dirs {
                std::fs::create_dir_all(dir).unwrap();
            }

            dirs
        }};
    }

    #[test]
    fn finds_git_dirs() {
        let code_dir_test = "finds_git_dirs_test";
        create_dirs!(
            code_dir_test,
            vec![
                "notaproject",
                "aproject/.git",
                "anotherproject/.git",
                "nope",
            ]
        );

        let finder = Finder::from(code_dir_test);
        let mut expected = vec!["aproject", "anotherproject"]
            .iter()
            .map(|s| {
                let mut ex = code_dir_test.to_string();
                ex.push_str("/");
                ex.push_str(s);
                ex
            })
            .collect::<Vec<String>>();
        expected.sort();

        let mut found = finder
            .map(|p| p.into_os_string().into_string())
            .map(|r| r.unwrap())
            .collect::<Vec<String>>();
        found.sort();

        assert_eq!(expected, found);

        std::fs::remove_dir_all(code_dir_test).unwrap();
    }

    #[test]
    fn excludes_dirs() {
        let code_dir_test = "excludes_git_dirs_test";
        create_dirs!(
            code_dir_test,
            vec![
                "notaproject",
                "aproject/.git",
                "anotherproject/.git",
                "ignored/.git",
                "ignored_but_included/.git",
                "nope",
            ]
        );

        let finder = Finder::from(
            Config::new(vec![code_dir_test.to_string()]).with_excludes(vec!["ignored".to_string()]),
        );

        let mut expected = vec!["aproject", "anotherproject"]
            .iter()
            .map(|s| {
                let mut ex = code_dir_test.to_string();
                ex.push_str("/");
                ex.push_str(s);
                ex
            })
            .collect::<Vec<String>>();
        expected.sort();

        let mut found = finder
            .map(|p| p.into_os_string().into_string())
            .map(|r| r.unwrap())
            .collect::<Vec<String>>();
        found.sort();

        assert_eq!(expected, found);

        std::fs::remove_dir_all(code_dir_test).unwrap();
    }

    #[test]
    fn excludes_and_includes_dirs() {
        let code_dir_test = "excludes_and_includes_git_dirs_test";
        create_dirs!(
            code_dir_test,
            vec![
                "notaproject",
                "aproject/.git",
                "anotherproject/.git",
                "ignored/.git",
                "ignored_but_included/.git",
                "nope",
            ]
        );

        let finder = Finder::from(
            Config::new(vec![code_dir_test.to_string()])
                .with_excludes(vec!["ignored".to_string()])
                .with_includes(vec!["included".to_string()]),
        );

        let mut expected = vec!["aproject", "anotherproject", "ignored_but_included"]
            .iter()
            .map(|s| {
                let mut ex = code_dir_test.to_string();
                ex.push_str("/");
                ex.push_str(s);
                ex
            })
            .collect::<Vec<String>>();

        expected.sort();

        let mut found = finder
            .map(|p| p.into_os_string().into_string())
            .map(|r| r.unwrap())
            .collect::<Vec<String>>();

        found.sort();

        assert_eq!(expected, found);
        std::fs::remove_dir_all(code_dir_test).unwrap();
    }
}
