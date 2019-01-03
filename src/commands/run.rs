use crate::find::projects::Finder;
use std::process::{exit, Command};

pub const USAGE: &str = "
Usage: run [options] [<args>...]

Options:
  --verbose  Print verbose information while finding repositories

List code repositories found in your code directories.

See 'projector help' for more information.
";

#[derive(Deserialize, Debug)]
pub struct Args {
    flag_verbose: bool,
    arg_args: Vec<String>,
}

pub fn run(finder: Finder, args: &Args) {
    match args.arg_args.split_first() {
        Some((program, arguments)) => {
            let mut child = Command::new(program);
            child.args(arguments);

            for project in finder {
                println!("\n\n{}:", project.to_string_lossy());
                let mut proc = child
                    .current_dir(project)
                    .spawn()
                    .expect("failed to start process");
                proc.wait().expect("failed to execute child process");
            }
        }
        None => {
            println!("ERROR: No command given");
            exit(1);
        }
    }
}
