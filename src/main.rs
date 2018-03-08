// Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved.
// Use of this source code is governed by the GPLv3 license that can be found in
// the LICENSE file.

extern crate ansi_term;
extern crate clap;
extern crate regex;
extern crate walkdir;

use std::env;
use std::process;
use std::process::Command;

use std::path::Path;

use std::fs::{metadata, File, OpenOptions};
use std::time::Duration;
use std::io::ErrorKind;
use std::io::prelude::*;

use std::sync::{Arc, Mutex};

use clap::{App, AppSettings, Arg, SubCommand};

use walkdir::WalkDir;

fn handle_error(e: walkdir::Error) -> bool {
    if let Some(path) = e.path() {
        if path.is_file() {
            return true;
        }
    }

    if let Some(err) = e.io_error() {
        if err.kind() == ErrorKind::NotFound {
            return true;
        }
    }

    println!("ERROR: {}", e);
    return false;
}

fn find_projects<F>(code_dir: String, callback: F)
where
    F: Fn(String) -> (),
{
    let wkd = WalkDir::new(code_dir);
    let git_dirs = wkd.into_iter();

    for dir in git_dirs {
        let cwd;
        match dir {
            Ok(d) => cwd = d,
            Err(e) => {
                if handle_error(e) {
                    continue;
                } else {
                    process::exit(1);
                }
            }
        };

        if !cwd.file_type().is_dir() {
            continue;
        }

        let file_name = cwd.file_name().to_str().unwrap_or("");
        if file_name == ".git" {
            let parent_path = cwd.path()
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

fn list_from_cache(cache_file: &Path) {
    let mut f = File::open(&cache_file).expect("Cache file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Unable to read from cache file.");
    println!("{}", contents)
}

fn list_from_fs(code_dir: String, cache_file: &Path) {
    let projects = Arc::new(Mutex::new(Vec::new()));

    find_projects(code_dir, |p| {
        println!("{}", p);
        projects.lock().unwrap().push(p.clone())
    });

    let results = projects.lock().unwrap().join("\n");

    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .open(cache_file)
        .expect("Unable to open cache file");

    match f.write_all(&results.into_bytes()) {
        Ok(_) => {}
        Err(e) => println!("ERROR: Unable to write cache file {}", e),
    }
}

fn should_use_cache(cache_file: &Path) -> bool {
    // Obviously if there's no cache we shouldn't use it.
    if !cache_file.exists() {
        return false
    }

    let cache_file_md = metadata(cache_file);

    match cache_file_md {
        Ok(md) => {
            let last_cache_access;
            if let Ok(time) = md.modified() {
                last_cache_access = time.elapsed().unwrap_or(Duration::new(0, 0));
                // 172800 is 48 hours in seconds. Rust doesn't have a better 
                // way to work with time.
                return last_cache_access.as_secs() < 172800;
            }

            true
        }
        Err(_) => false,
    }
}

fn list(code_dir: String, ignore_cache: bool) {
    let mut home = env::var("HOME").unwrap_or("".to_string());
    home.push_str("/.projector_cache");
    let cache_file = Path::new(&home);

    let use_cache = !ignore_cache && should_use_cache(&cache_file);
    if use_cache {
        list_from_cache(&cache_file)
    } else {
        list_from_fs(code_dir, cache_file)
    }
}

fn main() {
    let matches = App::new("projector")
        .version("0.1.2")
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
        .subcommand(
            SubCommand::with_name("list")
                .arg(Arg::with_name("no-cache").short("n").long("no-cache"))
                .arg(
                    Arg::with_name("refresh-cache")
                        .short("r")
                        .long("refresh-cache"),
                ),
        )
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

    if let Some(args) = matches.subcommand_matches("list") {
        list(
            code_dir,
            args.is_present("no-cache") || args.is_present("refresh-cache"),
        );
    } else if let Some(args) = matches.subcommand_matches("run") {
        let argv: Vec<&str> = args.values_of("ARGV").unwrap().collect();
        let cmd: Vec<String> = argv.iter().map(|x| x.to_string()).collect();
        run(code_dir, cmd);
    } else {
        println!("ERROR: Unknown command.");
    }
}
