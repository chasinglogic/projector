// Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved. Use of this source code is
// governed by the Apache-2.0 license that can be found in the LICENSE file.

extern crate clap;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::{exit, Command};

use clap::{App, AppSettings, Arg, SubCommand};

use dirs::home_dir;

use projector::find::projects::{Config, Finder};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn list(finder: Finder) {
    for project in finder {
        match writeln!(
            io::stdout().lock(),
            "{}",
            project.as_os_str().to_string_lossy()
        ) {
            Ok(()) => (),
            Err(_) => (),
        };
    }
}

fn run(finder: Finder, matches: &clap::ArgMatches) {
    let command: Vec<&str> = matches
        .values_of("COMMAND")
        .unwrap_or(clap::Values::default())
        .collect();

    match command.split_first() {
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

fn main() {
    let matches = App::new("projector")
        .version(VERSION)
        .about("A code repository manager.")
        .author("Mathew Robinson (@chasinglogic)")
        .arg(Arg::with_name("verbose").help("Enable verbosity options for all commands"))
        .arg(
            Arg::with_name("exclude")
                .short("e")
                .value_name("PATTERN")
                .multiple(true)
                .help("A regex which will be used to exclude directories from commands."),
        )
        .arg(
            Arg::with_name("include")
                .short("i")
                .value_name("PATTERN")
                .multiple(true)
                .help(
                    "A regex which will be used to include
                directories from commands. Overrides excludes so if a
                directory is matched by an exclude pattern and an
                include pattern the directory will be included.",
                ),
        )
        .arg(
            Arg::with_name("code-dir")
                .short("c")
                .value_name("DIRECTORY")
                .multiple(true)
                .help(
                    "The root of where to search for projects. Also
                     can be configured using the environment
                     variable CODE_DIR.",
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("ls")
                .help("List all projects that projector would operate on."),
        )
        .subcommand(
            SubCommand::with_name("run")
                .alias("x")
                .help("Execute command on all matched repos.")
                .setting(AppSettings::TrailingVarArg)
                .arg(Arg::with_name("COMMAND")),
        )
        .get_matches();

    let homedir = home_dir().unwrap_or_default();
    let mut config_file = homedir.clone();
    config_file.push(".projector.yml");

    // Used for simple $HOME tilde expansion
    let homedir_s = homedir.to_str().unwrap_or("");

    let mut config = if let Some(code_dirs) = matches.values_of("code-dir") {
        Config::new(code_dirs.map(|s| s.to_string()).collect::<Vec<String>>())
    } else if let Ok(mut cfg) = File::open(config_file) {
        let mut contents = String::new();
        if let Err(e) = cfg.read_to_string(&mut contents) {
            println!("Unable to read config file: {}", e);
            exit(1);
        }

        match serde_yaml::from_str(&contents) {
            Ok(c) => c,
            Err(e) => {
                println!("ERROR: Unable to deserialize config file. Maybe missing code_dir key?");
                println!("Full error: {}", e);
                exit(1);
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

    if let Some(excludes) = matches.values_of("excludes") {
        let mut patterns = excludes.map(|s| s.to_string()).collect::<Vec<String>>();
        if let Some(mut existing_excludes) = config.excludes {
            existing_excludes.append(&mut patterns);
            config.excludes = Some(existing_excludes);
        } else {
            config.excludes = Some(patterns);
        }
    }

    if let Some(includes) = matches.values_of("includes") {
        let mut patterns = includes.map(|s| s.to_string()).collect::<Vec<String>>();
        if let Some(mut existing_includes) = config.includes {
            existing_includes.append(&mut patterns);
            config.includes = Some(existing_includes);
        } else {
            config.includes = Some(patterns);
        }
    }

    let finder = Finder::from(config);

    if let Some(_) = matches.subcommand_matches("list") {
        list(finder);
        return;
    }

    if let Some(matches) = matches.subcommand_matches("run") {
        run(finder, matches);
        return;
    }
}
