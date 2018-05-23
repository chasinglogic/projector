// Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved.
// Use of this source code is governed by the GPLv3 license that can be found in
// the LICENSE file.

extern crate ansi_term;
extern crate clap;
extern crate regex;
extern crate serde;
extern crate walkdir;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod commands;
mod projects;

use clap::{App, AppSettings, Arg, SubCommand};
use std::env;
use std::path::PathBuf;

fn new_config(code_dir: String) -> projects::Config {
    let mut home = env::var("HOME").unwrap_or("".to_string());
    home.push_str("/.projector_cache");
    let cache_file = PathBuf::from(home);
    let config = projects::Config::new(code_dir).cache_file(cache_file);
    config
}

fn load_config(code_dir: String) -> projects::Config {
    let mut home = env::var("HOME").unwrap_or("".to_string());
    home.push_str(".projector.yml");

    if let Ok(config_file) = File::open(home) {
        let mut contents = String::new();
        config_file.read_to_string(&mut contents);
        let config: projects::Config = serde_yaml::from_str(&contents).unwrap();
        return config;
    }

    let config = new_config(code_dir);
    let config_file = File::create(home).unwrap();
    let content = serde_yaml::to_string(&config).unwrap();
    config_file.write_all(content.as_bytes());
    config
}

fn main() {
    let matches = App::new("projector")
        .version("0.1.3")
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
        .arg(Arg::with_name("no-cache").short("n").long("no-cache"))
        .arg(
            Arg::with_name("refresh-cache")
                .short("r")
                .long("refresh-cache"),
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

    let ignore_cache = matches.is_present("no-cache") || matches.is_present("refresh-cache");
    let config = load_config(code_dir);
    config.ignore_cache = ignore_cache || config.ignore_cache;

    if let Some(_args) = matches.subcommand_matches("list") {
        commands::list(config);
    } else if let Some(args) = matches.subcommand_matches("run") {
        let argv: Vec<&str> = args.values_of("ARGV").unwrap().collect();
        let cmd: Vec<String> = argv.iter().map(|x| x.to_string()).collect();
        commands::run(config, cmd);
    } else {
        println!("ERROR: Unknown command.");
    }
}
