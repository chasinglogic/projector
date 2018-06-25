use std::io::ErrorKind;
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

pub fn find<F>(code_dir: String, callback: F)
where
    F: Fn(String) -> (),
{
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
            callback(dir.path().to_str().unwrap_or("").to_string());
            wkd.skip_current_dir();
        }
    }
}
