use std::fs::DirEntry;

use regex::RegexSet;

pub struct Filters {
    excludes: RegexSet,
    includes: RegexSet,
}

impl Filters {
    pub fn new() -> Filters {
        Filters {
            excludes: RegexSet::new(&["^$"]).unwrap(),
            // guaranteed to compile to safe so unwrap
            includes: RegexSet::new(&["^$"]).unwrap(),
        }
    }

    pub fn with_excludes(mut self, excludes: RegexSet) -> Filters {
        self.excludes = excludes;
        self
    }

    pub fn with_includes(mut self, includes: RegexSet) -> Filters {
        self.includes = includes;
        self
    }

    pub fn should_skip(&self, entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| {
                s.starts_with(".") || (self.excludes.is_match(&s) && !self.includes.is_match(&s))
            })
            .unwrap_or(false)
    }
}
