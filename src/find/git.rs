use std::path::Path;
use std::process::Command;

pub fn repo_is_dirty(repo: &Path) -> bool {
    if repo_has_uncommitted_or_unstaged_changes(repo) {
        return true;
    }

    if repo_has_untracked_files(repo) {
        return true;
    }

    false
}

fn repo_has_uncommitted_or_unstaged_changes(repo: &Path) -> bool {
    let proc = Command::new("git")
        .args(["diff-index", "--quiet", "HEAD", "--"])
        .current_dir(repo)
        .spawn()
        .expect("Unable to run git!")
        .wait();

    if let Ok(exit) = proc {
        // exit code 0 means no uncommitted changes so we invert here.
        !exit.success()
    } else {
        false
    }
}

fn repo_has_untracked_files(repo: &Path) -> bool {
    let mut proc = Command::new("git");
    proc.args(["ls-files", "--exclude-standard", "--others"]);
    proc.current_dir(repo);

    let output = proc
        .output()
        .expect("Unable to get output of git ls-files!");

    output.stdout.len() != 0
}
