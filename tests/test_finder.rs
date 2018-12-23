extern crate projector;

use projector::find::projects::{Config, Finder};
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

    let finder = Config::new(vec![code_dir_test.to_string()])
        .with_excludes(vec!["ignored".to_string()])
        .finder();

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

    let finder = Config::new(vec![code_dir_test.to_string()])
        .with_excludes(vec!["ignored".to_string()])
        .with_includes(vec!["included".to_string()])
        .finder();
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
    println!("{:?}", found);

    found.sort();

    assert_eq!(expected, found);
    std::fs::remove_dir_all(code_dir_test).unwrap();
}
