extern crate projector;

use projector::finder::Finder;
use std::path::{Path, PathBuf};

macro_rules! create_dirs {
    ($dirs:expr) => {
        let dirs: Vec<PathBuf> = $dirs.iter().map(PathBuf::from).collect();
        for dir in &dirs {
            std::fs::create_dir_all(dir).unwrap();
        }
    };
}

#[test]
fn finds_git_dirs() {
    let code_dir_test = "finds_git_dirs_code_dir_test";
    std::fs::remove_dir_all(code_dir_test);

    create_dirs!(vec![
        "finds_git_dirs_code_dir_test/notaproject",
        "finds_git_dirs_code_dir_test/aproject/.git",
        "finds_git_dirs_code_dir_test/anotherproject/.git",
        "finds_git_dirs_code_dir_test/nope",
    ]);

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
