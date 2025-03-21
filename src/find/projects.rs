use regex::RegexSet;
use std::fs::read_dir;
use std::path::PathBuf;
use std::process;

use super::config::Config;
use super::git::repo_is_dirty;

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

pub struct Finder {
    excludes: RegexSet,
    includes: RegexSet,
    dirty_only: bool,
    candidates: Vec<PathBuf>,
    matches: Vec<PathBuf>,
}

impl Finder {
    pub fn new(code_dirs: Vec<PathBuf>) -> Finder {
        Finder {
            candidates: code_dirs,
            matches: Vec::new(),
            // guaranteed to compile so safe so unwrap
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
}

impl From<Config> for Finder {
    fn from(cfg: Config) -> Finder {
        let path_bufs = cfg.code_dirs.into_iter().map(PathBuf::from).collect();
        Finder::new(path_bufs)
            .with_excludes(regex_from_patterns(cfg.excludes.unwrap_or_default()))
            .with_includes(regex_from_patterns(cfg.includes.unwrap_or_default()))
    }
}

impl Iterator for Finder {
    type Item = PathBuf;

    fn next(&mut self) -> Option<PathBuf> {
        loop {
            // Clear out the matches we've found so far.
            if let Some(project) = self.matches.pop() {
                return Some(project);
            }

            // Check if we have a new candidate directory to search for projects.
            let root_dir = match self.candidates.pop() {
                Some(candidate) => candidate,
                // If we have no matches and no candidates our work is done.
                None => return None,
            };

            let entries = match read_dir(root_dir) {
                Ok(iter) => iter,
                Err(_) => continue,
            };

            let mut new_candidates = Vec::new();
            for entry in entries {
                let entry = entry.expect("couldn't get dir entry");
                let path = entry.path();
                let path_string = path.to_str().unwrap_or_default();
                if self.excludes.is_match(path_string) && !self.includes.is_match(path_string) {
                    continue;
                }

                if path.is_dir() {
                    let mut git_path = path.clone();
                    git_path.push(".git");
                    if let Ok(true) = git_path.try_exists() {
                        if self.dirty_only && !repo_is_dirty(&path) {
                            continue;
                        }

                        self.matches.push(path);
                    } else {
                        new_candidates.push(path);
                    }
                }
            }

            self.candidates.extend(new_candidates.into_iter());
        }
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

        let finder = Finder::new(vec![PathBuf::from(code_dir_test)]);
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
