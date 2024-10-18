use serde::Deserialize;

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
