// Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved.
// Use of this source code is governed by the GPLv3 license that can be found in
// the LICENSE file.

extern crate clap;
extern crate projector;
extern crate serde_yaml;

use clap::{App, AppSettings, Arg, SubCommand};
use projector::projects::{find, Config};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;
use std::process::Command;

fn run(config: Config, command: Vec<String>) {
    if let Some((program, arguments)) = command.split_first() {
        find(config, |p| {
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

fn list(config: Config) {
    find(config, |p| println!("{}", p))
}

fn main() {
    let matches = App::new("projector")
        .version("0.2.0")
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
        .arg(
            Arg::with_name("exclude")
                .short("e")
                .long("exclude")
                .value_name("PATTERN")
                .takes_value(true)
                .help("A regex which will be used to exclude directories from commands."),
        )
        .arg(
            Arg::with_name("include")
                .short("i")
                .long("include")
                .value_name("PATTERN")
                .takes_value(true)
                .help(
                    "A regex which will be used to include directories from commands. Overrides
excludes so if a directory is matched by an exclude pattern and an include
pattern the directory will be included.",
                ),
        )
        .subcommand(SubCommand::with_name("list"))
        .subcommand(
            SubCommand::with_name("run")
                .setting(AppSettings::TrailingVarArg)
                .arg(Arg::with_name("ARGV").multiple(true).default_value("")),
        )
        .get_matches();

    let mut config = Config {
        code_dir: "~/Code".to_string(),
        includes: None,
        excludes: None,
    };

    let mut config_file = env::home_dir().unwrap_or(PathBuf::new());
    config_file.push(".projector.yml");
    if let Ok(mut cfg) = File::open(config_file) {
        let mut contents = String::new();
        cfg.read_to_string(&mut contents)
            .expect("unable to read config file");
        config = match serde_yaml::from_str(&contents) {
            Ok(c) => c,
            Err(_) => {
                println!("ERROR: Unable to deserialize config file. Maybe missing code_dir?");
                process::exit(1);
            }
        }
    }

    config.code_dir = if let Some(dir) = matches.value_of("code-dir") {
        dir.to_string()
    } else if let Ok(dir) = env::var("CODE_DIR") {
        dir
    } else {
        config.code_dir
    };

    if let Some(pattern) = matches.value_of("exclude") {
        if let Some(mut excludes) = config.excludes {
            excludes.push(pattern.to_string());
            config.excludes = Some(excludes);
        } else {
            config.excludes = Some(vec![pattern.to_string()]);
        }
    }

    if let Some(pattern) = matches.value_of("include") {
        if let Some(mut includes) = config.includes {
            includes.push(pattern.to_string());
            config.includes = Some(includes);
        } else {
            config.includes = Some(vec![pattern.to_string()]);
        }
    }

    // Simple $HOME tilde expansion
    if config.code_dir.starts_with("~") {
        config.code_dir = config.code_dir.replacen(
            "~",
            &env::home_dir()
                .expect("ERROR: Unable to find home dir.")
                .to_str()
                .unwrap_or("")
                .to_string(),
            1,
        )
    }

    if let Some(_) = matches.subcommand_matches("list") {
        list(config);
    } else if let Some(args) = matches.subcommand_matches("run") {
        let argv: Vec<&str> = args.values_of("ARGV").unwrap().collect();
        let cmd: Vec<String> = argv.iter().map(|x| x.to_string()).collect();
        run(config, cmd);
    } else {
        println!("ERROR: Unknown command.");
    }
}
