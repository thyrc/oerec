#![allow(clippy::module_name_repetitions)]
use postgres::Client;
use std::ffi::OsStr;
use std::fs::{self, remove_dir_all, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

use crate::exit_with_message;

use crate::generate::generate_serverauth;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with('.'))
}

pub fn write_serverauth(pgclient: &mut Client, dir: Option<&OsStr>, force: bool) {
    let serverauth = generate_serverauth(pgclient, None);

    let workdir = match dir {
        Some(dir) => PathBuf::from(dir),
        _ => exit_with_message("Could not write authorized_keys."),
    };

    if workdir.is_dir() && !force {
        print!(
            "Directory '{}' already exists. Do you want to delete *all* entries in the tree? [y/N]: ",
            &workdir.display()
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    let walker = WalkDir::new(&workdir).min_depth(1).max_depth(1).into_iter();
    for dir in walker.filter_entry(|e| !is_hidden(e)).flatten() {
        if remove_dir_all(dir.path()).is_err() {
            exit_with_message("Could not clean workdir.");
        }
    }

    for auth in serverauth {
        let outdir = workdir.join(&auth.serverip).join(&auth.sshuser.user);

        if fs::create_dir_all(&outdir).is_err() {
            exit_with_message("Could not write authorized_keys.");
        }

        for key in &auth.sshuser.authorized_keys.keys {
            let mut file = match OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(outdir.join("authorized_keys"))
            {
                Ok(f) => f,
                _ => exit_with_message("Could not write authorized_keys."),
            };

            if file.write(key.as_bytes()).is_err() {
                exit_with_message("Could not write authorized_keys.");
            };

            if file.write_all("\n".as_bytes()).is_err() {
                exit_with_message("Could not write authorized_keys.");
            }
        }
    }
}
