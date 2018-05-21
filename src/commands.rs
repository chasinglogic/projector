use projects::find_projects;
use std::process::Command;

pub fn run(code_dir: String, ignore_cache: bool, command: Vec<String>) {
    if let Some((program, arguments)) = command.split_first() {
        find_projects(code_dir, ignore_cache, |p| {
            println!("\n{}", "=".repeat(80));
            println!("Running in: {}", p);
            println!("{}\n", "=".repeat(80));
            let child = Command::new(program).args(arguments).current_dir(p).spawn();

            match child {
                Ok(mut subproc) => {
                    if let Err(e) = subproc.wait() {
                        println!("{}", e);
                    }
                }
                Err(e) => println!("Error spawning process: {}", e),
            }
        })
    } else if command.len() == 0 {
        println!("ERROR: No command given");
    }
}

pub fn list(code_dir: String, ignore_cache: bool) {
    find_projects(code_dir, ignore_cache, |p| {
        println!("{}", p);
    });
}
