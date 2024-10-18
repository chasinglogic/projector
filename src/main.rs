// Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved.
// Use of this source code is governed by the Apache-2.0 license that can be
// found in the LICENSE file.

mod commands;
pub mod find;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::{exit, Command};

use clap::{Parser, Subcommand};

use dirs::home_dir;

use commands::find;
use find::config::Config;
use find::projects::Finder;

fn list(finder: Finder) {
    for project in finder {
        writeln!(
            io::stdout().lock(),
            "{}",
            project.as_os_str().to_string_lossy()
        )
        .unwrap_or(());
    }
}

fn run(finder: Finder, command: Vec<String>) {
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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[arg(short, long)]
    excludes: Vec<String>,
    #[arg(short, long)]
    includes: Vec<String>,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    #[arg(short, long)]
    code_dir: Vec<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all projects that projector would operate on.
    #[clap(aliases = &["l", "ls"])]
    List {
        /// Only show projects with a dirty git state.
        #[arg(short, long)]
        dirty: bool,
    },

    /// Find projects by matching their paths.
    ///
    /// If multiple projects match then the rightmost match will be printed.
    ///
    /// If --verbose is provided then all matches will be printed.
    #[clap(aliases = &["search", "f"], trailing_var_arg = true)]
    Find {
        /// If provided find leftmost match instead of rightmost match.
        #[arg(short, long)]
        reverse: bool,

        #[arg()]
        search: Vec<String>,
    },

    /// Run a command on all matching projects
    #[clap(alias = "x", trailing_var_arg = true)]
    Run {
        #[arg()]
        command: Vec<String>,
    },
}

fn main() {
    let matches = CLI::parse();

    let homedir = home_dir().unwrap_or_default();
    let mut config_file = homedir.clone();
    config_file.push(".projector.yml");

    // Used for simple $HOME tilde expansion
    let homedir_s = homedir.to_str().unwrap_or("");

    let mut config = if matches.code_dir.len() > 0 {
        Config::new(matches.code_dir)
    } else if let Ok(mut cfg) = File::open(config_file) {
        let mut contents = String::new();
        if let Err(e) = cfg.read_to_string(&mut contents) {
            println!("Unable to read config file: {}", e);
            exit(1);
        }

        let mut c: Config = match serde_yml::from_str(&contents) {
            Ok(c) => c,
            Err(e) => {
                println!("ERROR: Unable to deserialize config file. Maybe missing code_dir key?");
                println!("Full error: {}", e);
                exit(1);
            }
        };

        c.code_dirs = c
            .code_dirs
            .iter()
            .map(|s| s.replacen("~", homedir_s, 1))
            .collect();
        c
    } else {
        Config::from(if let Ok(val) = std::env::var("CODE_DIR") {
            val
        } else {
            format!("{}/Code", homedir_s)
        })
    };

    if matches.excludes.len() > 0 {
        config = config.with_excludes(matches.excludes);
    }

    if matches.includes.len() > 0 {
        config = config.with_includes(matches.includes);
    }

    let finder = Finder::from(config);

    match matches.command {
        Some(Commands::List { dirty }) => list(finder.with_dirty_only(dirty)),
        Some(Commands::Run { command }) => run(finder, command),
        Some(Commands::Find { reverse, search }) => {
            if search.len() == 0 {
                println!("Must provide a search. For list using `projector list`");
                exit(1);
            }

            let search_term = search.join(" ");
            find(finder, search_term, reverse, matches.verbose);
        }
        None => {
            println!("Unknown subcommand.");
            exit(1);
        }
    }
}
