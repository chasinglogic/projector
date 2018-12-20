// Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved. Use of this source code is
// governed by the Apache-2.0 license that can be found in the LICENSE file.

#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate projector;

use std::env;
use std::process;
use std::process::Command;

use docopt::Docopt;

use projector::commands;
use projector::finder::Finder;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const USAGE: &str = "
Usage: projector [options] <command> [<args>...]

A code repository manager. Automate working across repositories.

Options:
  -h, --help      Print this help message
  --version       Print version and license information
  -v, --verbose   Print debug information, useful when submitting bug reports!
  -c, --code-dir <dir>...    The root of where to search for projects. Also can be
                             configured using the environment variable CODE_DIR.
                             default: ~/Code

  -e, --exclude <pattern>         A regex which will be used to exclude directories from commands.
  -i, --include <pattern>         A regex which will be used to include directories from commands. Overrides
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
    flag_code_dir: String,
    flag_exclude: Option<String>,
    flag_include: Option<String>,
    arg_command: String,
    arg_args: Vec<String>,
}

#[inline]
fn alias(cmd: &str) -> &str {
    match cmd {
        "l" => "list",
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

        let finder = Finder::from(".");

        match subcommand {
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

        "nope", "nope",
    }
}

// // Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved.
// // Use of this source code is governed by the GPLv3 license that can be found in
// // the LICENSE file.

// extern crate dirs;
// extern crate docopt;
// extern crate projector;
// extern crate serde_yaml;
// // extern crate rayon;

// use clap::{App, AppSettings, Arg, SubCommand};
// use dirs::home_dir;
// use projector::projects::{find, Config};
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::PathBuf;
// use std::process;
// use std::process::Command;

// fn run(config: Config, command: Vec<String>) {
//     if let Some((program, arguments)) = command.split_first() {
//         find(config, |p| {
//             println!("\n\n{}:", p);
//             let mut child = Command::new(program)
//                 .args(arguments)
//                 .current_dir(p)
//                 .spawn()
//                 .expect("failed to start process");
//             child.wait().expect("failed to execute child process");
//             ()
//         })
//     } else if command.len() == 0 {
//         println!("ERROR: No command given");
//     }
// }

// fn list(config: Config) {
//     find(config, |p| println!("{}", p))
// }

// fn main() {
//     let matches = App::new("projector")
//         .version("0.2.0")
//         .author("Mathew Robinson <chasinglogic@gmail.com>")
//         .arg(
//             Arg::with_name("code-dir")
//                 .short("c")
//                 .long("code-dir")
//                 .value_name("CODE_DIR")
//                 .multiple(true)
//                 .takes_value(true)
//                 .help(
//                     "The root of where to search for projects. Also can be
// configured using the environment variable CODE_DIR.
// Default: ~/Code",
//                 ),
//         )
//         .arg(
//             Arg::with_name("exclude")
//                 .short("e")
//                 .long("exclude")
//                 .value_name("PATTERN")
//                 .takes_value(true)
//                 .help("A regex which will be used to exclude directories from commands."),
//         )
//         .arg(
//             Arg::with_name("include")
//                 .short("i")
//                 .long("include")
//                 .value_name("PATTERN")
//                 .takes_value(true)
//                 .help(
//                     "A regex which will be used to include directories from commands. Overrides
// excludes so if a directory is matched by an exclude pattern and an include
// pattern the directory will be included.",
//                 ),
//         )
//         .subcommand(SubCommand::with_name("list"))
//         .subcommand(
//             SubCommand::with_name("run")
//                 .setting(AppSettings::TrailingVarArg)
//                 .arg(Arg::with_name("ARGV").multiple(true).default_value("")),
//         )
//         .get_matches();

//     let homedir = home_dir().unwrap_or(PathBuf::new());
//     let mut config_file = homedir.clone();
//     config_file.push(".projector.yml");

//     let mut config = if let Some(code_dirs) = matches.values_of("code-dir") {
//         Config::new(code_dirs.map(|s| s.to_string()).collect::<Vec<String>>())
//     } else if let Ok(mut cfg) = File::open(config_file) {
//         let mut contents = String::new();
//         cfg.read_to_string(&mut contents)
//             .expect("unable to read config file");

//         match serde_yaml::from_str(&contents) {
//             Ok(c) => c,
//             Err(_) => {
//                 println!("ERROR: Unable to deserialize config file. Maybe missing code_dir?");
//                 process::exit(1);
//             }
//         }
//     } else {
//         Config::one("~/Code".to_string())
//     };

//     if let Some(pattern) = matches.value_of("exclude") {
//         if let Some(mut excludes) = config.excludes {
//             excludes.push(pattern.to_string());
//             config.excludes = Some(excludes);
//         } else {
//             config.excludes = Some(vec![pattern.to_string()]);
//         }
//     }

//     if let Some(pattern) = matches.value_of("include") {
//         if let Some(mut includes) = config.includes {
//             includes.push(pattern.to_string());
//             config.includes = Some(includes);
//         } else {
//             config.includes = Some(vec![pattern.to_string()]);
//         }
//     }

//     // Simple $HOME tilde expansion
//     let homedir_s = homedir.to_str().unwrap_or("");

//     config.code_dirs = config
//         .code_dirs
//         .iter()
//         .map(|s| s.replacen("~", homedir_s, 1))
//         .collect();

//     if let Some(_) = matches.subcommand_matches("list") {
//         list(config);
//     } else if let Some(args) = matches.subcommand_matches("run") {
//         let argv: Vec<&str> = args.values_of("ARGV").unwrap().collect();
//         let cmd: Vec<String> = argv.iter().map(|x| x.to_string()).collect();
//         run(config, cmd);
//     } else {
//         println!("ERROR: Unknown command.");
//     }
// }
