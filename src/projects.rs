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
        if err.kind() == ErrorKind::NotFound {
            return true;
        }
    }

    println!("ERROR: {}", e);
    return false;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub code_dir: String,
    pub excludes: Option<Vec<String>>,
    pub includes: Option<Vec<String>>,
}

pub fn find<F>(config: Config, callback: F)
where
    F: Fn(String) -> (),
{
    let mut pat;
    let exclude;
    let include;
    if let Some(ref patterns) = config.excludes {
        pat = "(".to_string();

        for pattern in patterns {
            pat.push_str(&pattern);
            pat.push_str("|");
        }

        // Remove trailing |
        pat.pop();
        pat.push_str(")");

        exclude = match Regex::new(&pat) {
            Ok(r) => r,
            Err(e) => {
                println!("ERROR: Unable to compile regex: {}: {}", pat, e);
                process::exit(1);
            }
        }
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

    let mut wkd = WalkDir::new(config.code_dir).into_iter();
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
}
