use home::home_dir;
use log::error;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, NoTls};
use serde_derive::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const DEFAULT_LOGFILE: &str = "/var/log/oerec.log";

#[macro_use]
extern crate prettytable;

mod commands;
mod key;
mod logging;
mod server;
mod serveraccess;
mod serverauth;
mod servergroup;
mod user;
mod useraccess;
mod usergroup;

#[derive(Deserialize, Debug)]
struct Config {
    client: ClientConfig,
    db: DBConfig,
    logging: Option<LogConfig>,
}

#[derive(Deserialize, Debug)]
struct ClientConfig {
    user: Option<String>,
    password: Option<String>,
}

#[derive(Deserialize, Debug)]
struct DBConfig {
    path: Option<String>,
    host: Option<String>,
    dbname: Option<String>,
}

#[derive(Deserialize, Debug)]
struct LogConfig {
    file: Option<PathBuf>,
}

#[derive(Copy, Clone, Debug)]
pub enum ListObject {
    UserEmail,
    UserName,
    UserGroup,
    ServerName,
    ServerGroup,
    ServerAccess,
    KeyID,
}

#[cfg(unix)]
pub fn pipe_reset() {
    unsafe {
        ::libc::signal(::libc::SIGPIPE, ::libc::SIG_DFL);
    }
}

#[cfg(not(unix))]
pub fn pipe_reset() {}

#[must_use]
pub fn set_or_ask_for(opt: Option<&str>, prompt: &str) -> String {
    if let Some(opt) = opt {
        (*opt).to_string()
    } else {
        let mut userinput = String::new();
        print!("{prompt}: ");
        if io::stdout().flush().is_err() {
            exit_with_message("Could not print to stdout.");
        };
        if io::stdin().read_line(&mut userinput).is_err() {
            exit_with_message("Could not read from stdin.");
        };
        userinput[..].trim().to_string()
    }
}

#[must_use]
pub fn ask_for(
    object: &ListObject,
    key: Option<&str>,
    message: Option<&str>,
    pgclient: &mut Client,
) -> String {
    let (prompt, mut lcmd) = match object {
        ListObject::UserEmail => (
            message.unwrap_or("User email ['?' for list]"),
            Box::new(|o: String| {
                _ = &user::list(pgclient, o.strip_suffix('?'), None, None, false, false);
            }) as Box<dyn FnMut(_)>,
        ),
        ListObject::UserName => (
            message.unwrap_or("User name ['?' for list]"),
            Box::new(|o: String| {
                _ = &user::list(pgclient, None, o.strip_suffix('?'), None, false, false);
            }) as Box<dyn FnMut(_)>,
        ),
        ListObject::ServerName => (
            message.unwrap_or("Server name ['?' for list]"),
            Box::new(|o: String| {
                _ = &server::list(pgclient, o.strip_suffix('?'), None, None, false, false);
            }) as Box<dyn FnMut(_)>,
        ),
        ListObject::UserGroup => (
            message.unwrap_or("User group name ['?' for list]"),
            Box::new(|o: String| {
                _ = &usergroup::list(pgclient, o.strip_suffix('?'), None, false, false, false);
            }) as Box<dyn FnMut(_)>,
        ),
        ListObject::KeyID => (
            message.unwrap_or("Key ID ['?' for list]"),
            Box::new(|o: String| {
                _ = &key::list(pgclient, o.strip_suffix('?'), None, false, None, false);
            }) as Box<dyn FnMut(_)>,
        ),
        ListObject::ServerGroup => (
            message.unwrap_or("Server group name ['?' for list]"),
            Box::new(|o: String| {
                _ = &servergroup::list(
                    pgclient,
                    o.strip_suffix('?'),
                    None,
                    None,
                    false,
                    false,
                    false,
                    false,
                );
            }) as Box<dyn FnMut(_)>,
        ),
        ListObject::ServerAccess => (
            message.unwrap_or("Server access name ['?' for list]"),
            Box::new(|o: String| {
                _ = &serveraccess::list(
                    pgclient,
                    o.strip_suffix('?'),
                    None,
                    None,
                    None,
                    None,
                    false,
                    false,
                );
            }) as Box<dyn FnMut(_)>,
        ),
    };

    #[allow(clippy::ignored_unit_patterns)]
    loop {
        let o = set_or_ask_for(key, prompt);
        if o.trim().ends_with('?') {
            _ = &lcmd(o);
        } else {
            return o;
        }
    }
}

fn exit_with_message(message: &str) -> ! {
    println!();
    eprintln!(
        "{} {}",
        "error:".if_supports_color(Stdout, owo_colors::OwoColorize::red),
        message
    );
    std::process::exit(1)
}

fn main() {
    pipe_reset();
    let homedir = match home_dir() {
        Some(path) => path,
        _ => PathBuf::from("/root"),
    };
    let configfile = match (
        homedir.join(r"oerec.toml").exists(),
        PathBuf::from(r"./").join(r"oerec.toml").exists(),
    ) {
        (true, _) => homedir.join(r"oerec.toml"),
        (_, _) => PathBuf::from(r"./oerec.toml"),
    };
    let contents = match fs::read_to_string(&configfile) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!(
                "Could not read config file '{}': {}",
                configfile.display(),
                e
            );
            std::process::exit(1);
        }
    };

    let config: Config = if let Ok(config) = toml::from_str(&contents) {
        config
    } else {
        exit_with_message("Could not parse config file.");
    };

    let mut pgconfig = postgres::config::Config::new();
    pgconfig
        .user(&config.client.user.unwrap_or_else(|| "oerec".to_string()))
        .host_path(
            config
                .db
                .path
                .unwrap_or_else(|| "/var/run/postgres".to_string()),
        )
        .host(&config.db.host.unwrap_or_else(|| "localhost".to_string()))
        .dbname(&config.db.dbname.unwrap_or_else(|| "oere".to_string()));

    if let Some(password) = config.client.password {
        pgconfig.password(&password);
    }

    let Ok(mut con) = pgconfig.connect(NoTls) else {
        exit_with_message("Could not connect to database.");
    };

    // logging
    let logfile = if let Some(logconfig) = config.logging {
        logconfig
            .file
            .unwrap_or_else(|| PathBuf::from(DEFAULT_LOGFILE))
    } else {
        PathBuf::from(DEFAULT_LOGFILE)
    };

    if let Err(e) = logging::create_logger(&logfile) {
        error!("Could create logger: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = commands::parse_subcommands(&mut con) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
