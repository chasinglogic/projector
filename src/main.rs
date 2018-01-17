// Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved.
// Use of this source code is governed by the GPLv3 license that can be found in
// the LICENSE file.

extern crate regex;
extern crate clap;
extern crate walkdir;
extern crate ansi_term;

use std::env;
use clap::{App, Arg, SubCommand, AppSettings};
use walkdir::WalkDir;
use std::io::ErrorKind;
use std::process;
use std::process::Command;


fn handle_error(e: walkdir::Error) -> bool {
    if let Some(path) = e.path() {
        if path.is_file() { return true }
    }

    if let Some(err) = e.io_error() {
        if err.kind() == ErrorKind::NotFound {
            return true
        }
    }

    println!("ERROR: {}", e);
    return false
}

fn find_projects<F>(code_dir: String, callback: F) where F: (Fn(String) -> ()) {
    let wkd = WalkDir::new(code_dir);
    let git_dirs = wkd.into_iter();

    for dir in git_dirs {
        let cwd;
        match dir {
            Ok(d) => cwd = d,
            Err(e) => {
                if handle_error(e) {
                    continue
                } else {
                    process::exit(1);
                }
            },
        };

        if !cwd.file_type().is_dir() { continue }

        let file_name = cwd.file_name().to_str().unwrap_or("");
        if file_name == ".git" {
            let parent_path = cwd
                .path()
                .parent()
                .unwrap()
                .to_str()
                .unwrap_or("")
                .to_string();

            callback(parent_path);
        }
    }
}

fn run(code_dir: String, command: Vec<String>) {
    if let Some((program, arguments)) = command.split_first() {
        find_projects(code_dir, |p| {
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
    find_projects(code_dir, |p| println!("{}", p))
}

fn main() {
    let matches = App::new("projector")
        .version("0.1.0")
        .author("Mathew Robinson <chasinglogic@gmail.com>")
        .arg(Arg::with_name("code-dir")
             .short("c")
             .long("code-dir")
             .value_name("CODE_DIR")
             .takes_value(true)
             .help("The root of where to search for projects. Also can be
configured using the environment variable CODE_DIR.
Default: ~/Code"))
        .subcommand(SubCommand::with_name("list"))
        .subcommand(SubCommand::with_name("run")
                    .setting(AppSettings::TrailingVarArg)
                    .arg(Arg::with_name("ARGV")
                         .multiple(true)
                         .default_value("")))
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
