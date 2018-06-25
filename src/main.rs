// Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved.
// Use of this source code is governed by the GPLv3 license that can be found in
// the LICENSE file.

extern crate clap;
extern crate projector;

use clap::{App, AppSettings, Arg, SubCommand};
use projector::projects::find;
use std::env;
use std::process::Command;

fn run(code_dir: String, command: Vec<String>) {
    if let Some((program, arguments)) = command.split_first() {
        find(code_dir, |p| {
            println!("\n\n{}:", p);
            let mut child = Command::new(program)
                .args(arguments)
                .current_dir(p)
                .spawn()
                .expect("failed to start process");
            child.wait().expect("failed to execute child process");
            ()
        })
    } else if command.len() == 0 {
        println!("ERROR: No command given");
    }
}

fn list(code_dir: String) {
    find(code_dir, |p| println!("{}", p))
}

fn main() {
    let matches = App::new("projector")
        .version("0.1.0")
        .author("Mathew Robinson <chasinglogic@gmail.com>")
        .arg(
            Arg::with_name("code-dir")
                .short("c")
                .long("code-dir")
                .value_name("CODE_DIR")
                .takes_value(true)
                .help(
                    "The root of where to search for projects. Also can be
configured using the environment variable CODE_DIR.
Default: ~/Code",
                ),
        )
        .subcommand(SubCommand::with_name("list"))
        .subcommand(
            SubCommand::with_name("run")
                .setting(AppSettings::TrailingVarArg)
                .arg(Arg::with_name("ARGV").multiple(true).default_value("")),
        )
        .get_matches();

    let code_dir: String = if let Some(dir) = matches.value_of("code-dir") {
        dir.to_string()
    } else if let Ok(dir) = env::var("CODE_DIR") {
        dir
    } else {
        "~/Code".to_string()
    };

    if let Some(_) = matches.subcommand_matches("list") {
        list(code_dir);
    } else if let Some(args) = matches.subcommand_matches("run") {
        let argv: Vec<&str> = args.values_of("ARGV").unwrap().collect();
        let cmd: Vec<String> = argv.iter().map(|x| x.to_string()).collect();
        run(code_dir, cmd);
    } else {
        println!("ERROR: Unknown command.");
    }
}
