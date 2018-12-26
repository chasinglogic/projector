// Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved. Use of this source code is
// governed by the Apache-2.0 license that can be found in the LICENSE file.

#[macro_use]
extern crate serde_derive;
extern crate dirs;
extern crate docopt;
extern crate projector;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;

use dirs::home_dir;
use docopt::Docopt;

use projector::commands;
use projector::find::projects::{Config, Finder};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const USAGE: &str = "
Usage: projector [options] <command> [<args>...]

A code repository manager. Automate working across repositories.

Options:
  -h, --help            Print this help message
  --version             Print version and license information
  -v, --verbose         Print debug information, useful when submitting bug reports!
  -c, --code-dir <dir>  The root of where to search for projects. Also can be
                        configured using the environment variable CODE_DIR.

  -e, --exclude <pattern>  A regex which will be used to exclude directories from commands.
  -i, --include <pattern>  A regex which will be used to include directories from commands. Overrides
                           excludes so if a directory is matched by an exclude pattern and an include
                           pattern the directory will be included.

Commands:
   help  Print this help message
   list  List projects found in your code directories
   run   Run a shell command in all of your code directories

See 'projector help <command>' for more information on a specific command.
";

#[derive(Deserialize, Debug)]
struct Args {
    flag_verbose: bool,
    flag_version: bool,
    flag_help: bool,
    flag_code_dirs: Option<Vec<String>>,
    flag_exclude: Option<String>,
    flag_include: Option<String>,
    arg_command: String,
    arg_args: Vec<String>,
}

#[inline]
fn alias(cmd: &str) -> &str {
    match cmd {
        "l" => "list",
        "r" => "run",
        x => x,
    }
}

fn main() {
    let mut args: Args = Docopt::new(USAGE)
        .and_then(|d| d.options_first(true).help(false).deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("projector version {}", VERSION);
    } else if args.arg_command == "" && args.flag_help {
        println!("projector version {}\n{}", VERSION, USAGE);
    } else {
        let subcommand = alias(&args.arg_command);
        let mut subc_args = vec![subcommand.to_string()];
        subc_args.append(&mut args.arg_args);

        let homedir = home_dir().unwrap_or(PathBuf::new());
        let mut config_file = homedir.clone();
        config_file.push(".projector.yml");

        // Used for simple $HOME tilde expansion
        let homedir_s = homedir.to_str().unwrap_or("");

        let mut config = if let Some(code_dirs) = args.flag_code_dirs {
            Config::new(code_dirs)
        } else if let Ok(mut cfg) = File::open(config_file) {
            let mut contents = String::new();
            if let Err(e) = cfg.read_to_string(&mut contents) {
                println!("Unable to read config file: {}", e);
                process::exit(1);
            }

            match serde_yaml::from_str(&contents) {
                Ok(c) => c,
                Err(e) => {
                    println!(
                        "ERROR: Unable to deserialize config file. Maybe missing code_dir key?"
                    );
                    println!("Full error: {}", e);
                    process::exit(1);
                }
            }
        } else {
            Config::from(format!("{}/Code", homedir_s))
        };

        config.code_dirs = config
            .code_dirs
            .iter()
            .map(|s| s.replacen("~", homedir_s, 1))
            .collect();

        let finder = Finder::from(config);

        match subcommand {
            "help" => {
                if subc_args.len() == 1 {
                    println!("projector version {}\n{}", VERSION, USAGE);
                    process::exit(0);
                }

                let c = alias(&subc_args[1]);
                println!(
                    "projector version {}\n{}",
                    VERSION,
                    match c {
                        "list" => commands::list::USAGE,
                        _ => {
                            println!("Unknown command: {}", c);
                            process::exit(1);
                        }
                    }
                );
            }
            "list" => {
                let args: commands::list::Args = Docopt::new(commands::list::USAGE)
                    .and_then(|d| {
                        d.argv(subc_args)
                            .options_first(true)
                            .help(false)
                            .deserialize()
                    })
                    .unwrap_or_else(|e| e.exit());

                commands::list::run(finder, args);
            }
            "run" => {
                let args: commands::run::Args = Docopt::new(commands::run::USAGE)
                    .and_then(|d| {
                        d.argv(subc_args)
                            .options_first(true)
                            .help(false)
                            .deserialize()
                    })
                    .unwrap_or_else(|e| e.exit());

                commands::run::run(finder, args);
            }
            _ => {
                println!("{}: is not a known subcommand", args.arg_command);
                process::exit(1);
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::alias;

    macro_rules! test_aliases {
        ($($alias:expr, $command:expr,)*) => {
            #[test]
            fn test_aliases() {
                $(
                    assert_eq!(alias($alias), $command);
                )*
            }
        }

    }

    test_aliases! {
        "l", "list",
        "r", "run",
        "nope", "nope",
    }
}
