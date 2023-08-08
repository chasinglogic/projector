use regex::RegexSet;

pub struct Filters {
    excludes: Option<RegexSet>,
    includes: RegexSet,
}

impl Filters {
    pub fn new() -> Filters {
        Filters {
            excludes: None,
            // guaranteed to compile to safe so unwrap
            includes: RegexSet::new(&["^$"]).unwrap(),
        }
    }

    pub fn with_excludes(mut self, excludes: RegexSet) -> Filters {
        self.excludes = Some(excludes);
        self
    }

    pub fn with_includes(mut self, includes: RegexSet) -> Filters {
        self.includes = includes;
        self
    }
}
