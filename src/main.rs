// Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights
// reserved. Use of this source code is governed by the Apache-2.0
// license that can be found in the LICENSE file.

mod find;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::{exit, Command};

use clap::{Parser, Subcommand};

use dirs::home_dir;

use find::projects::{Config, Finder};

fn last_match_percent(s: &str, rgx: &regex::Regex) -> f64 {
    let shortest_match = rgx.shortest_match(s).unwrap_or(1);
    s.len() as f64 / shortest_match as f64
}

fn find(finder: Finder, search_term: &str) {
    let rgx = match regex::Regex::new(search_term) {
        Ok(r) => r,
        Err(e) => {
            println!("Unable to compile regex: {}", e);
            exit(1);
        }
    };

    let mut matched_projects = Vec::new();
    for project in finder {
        let project_path = project.as_os_str().to_string_lossy();
        if rgx.is_match(&project_path) {
            matched_projects.push(project_path.to_string().clone());
        }
    }

    // let _reverse_search = matches.is_present("reverse");
    // if matches.is_present("verbose") {
    //     for project in matched_projects {
    //         println!("{}", project);
    //     }

    //     return;
    // }

    if matched_projects.is_empty() {
        println!("No projects matched that search.");
        return;
    }

    let mut shortest_path = "";
    let mut shortest_path_percentage = 0.0;

    for project in matched_projects.iter() {
        let percentage_match = last_match_percent(&project, &rgx);
        if percentage_match > shortest_path_percentage {
            shortest_path_percentage = percentage_match;
            shortest_path = project;
        }
    }

    println!("{}", shortest_path);
}

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
    /// A regex which will be used to include directories from
    /// commands. Overrides excludes so if a directory is matched by
    /// an exclude pattern and an include pattern the directory will
    /// be included.
    #[arg(short, long)]
    includes: Vec<String>,
    /// A regex which will be used to exclude directories from commands.
    #[arg(short, long)]
    excludes: Vec<String>,

    /// The root of where to search for projects. Also can be
    /// configured using the environment variable CODE_DIR.
    #[arg(short, long)]
    code_dir: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all projects projector would operate on.
    List {
        /// Only list projects with a dirty git state.
        #[arg(short, long)]
        dirty: bool,
    },

    /// Find projects by matching their paths. If multiple projects
    /// match SEARCH then prints the path with the rightmost match.
    ///
    /// If used with --verbose will print all matches.
    Find { search: String },
    // Command::with_name("run")
    //        .alias("x")
    //        .about("Execute command on all matched repos")
    //        .setting(AppSettings::TrailingVarArg)
    //        .arg(Arg::with_name("COMMAND").required(true).multiple(true)),

    // Run { }
}

fn main() {
    let matches = CLI::parse();

    let homedir = home_dir().unwrap_or_default();
    let mut config_file = homedir.clone();
    config_file.push(".projector.yml");

    // Used for simple $HOME tilde expansion
    let homedir_s = homedir.to_str().unwrap_or("");

    let mut config = if let Some(code_dirs) = matches.code_dir.as_deref() {
        Config::new(vec![code_dirs.to_string()])
    } else if let Ok(mut cfg) = File::open(config_file) {
        let mut contents = String::new();
        if let Err(e) = cfg.read_to_string(&mut contents) {
            println!("Unable to read config file: {}", e);
            exit(1);
        }

        let mut c: Config = match serde_yaml::from_str(&contents) {
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
        Config::from(format!("{}/Code", homedir_s))
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
        Some(Commands::Find { search }) => find(finder, &search),
        // ("run", Some(args)) => run(finder, args),
        None => {
            println!("Unknown subcommand.");
            exit(1);
        }
    }
}
