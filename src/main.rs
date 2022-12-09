use base64::{decode, encode};
use home::home_dir;
use log::error;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, NoTls};
use serde_derive::Deserialize;
use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const DEFAULT_LOGFILE: &str = "/var/log/oerec.log";

#[macro_use]
extern crate prettytable;

mod add;
mod cli;
mod delete;
mod generate;
mod list;
mod logging;
mod maintenance;
mod schema;
mod update;
mod write;

use crate::list::{
    list_key, list_server, list_serveraccess, list_serverauth, list_servergroup, list_user,
    list_useraccess, list_usergroup,
};

use crate::add::{
    add_key, add_server, add_server_to_servergroup, add_serveraccess, add_servergroup,
    add_servergroup_to_servergroup, add_user, add_user_to_usergroup, add_useraccess, add_usergroup,
    add_usergroup_to_usergroup,
};

use crate::delete::{
    delete_key, delete_server, delete_server_from_servergroup, delete_serveraccess,
    delete_servergroup, delete_servergroup_from_servergroup, delete_user,
    delete_user_from_usergroup, delete_useraccess, delete_usergroup,
    delete_usergroup_from_usergroup,
};

use crate::logging::create_logger;

use crate::maintenance::{
    disable_dns, disable_server, disable_user, enable_dns, enable_server, enable_user,
};

use crate::update::{
    update_key, update_server, update_serveraccess, update_servergroup, update_user,
    update_usergroup,
};

use crate::write::write_serverauth;

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

#[must_use]
pub fn set_or_ask_for(opt: Option<&str>, prompt: &str) -> String {
    if let Some(opt) = opt {
        (*opt).to_string()
    } else {
        let mut userinput = String::new();
        print!("{}: ", prompt);
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
                let _ = &list_user(pgclient, o.strip_suffix('?'), None, None, false, false);
            }) as Box<dyn FnMut(_)>,
        ),
        ListObject::UserName => (
            message.unwrap_or("User name ['?' for list]"),
            Box::new(|o: String| {
                let _ = &list_user(pgclient, None, o.strip_suffix('?'), None, false, false);
            }) as Box<dyn FnMut(_)>,
        ),
        ListObject::ServerName => (
            message.unwrap_or("Server name ['?' for list]"),
            Box::new(|o: String| {
                let _ = &list_server(pgclient, o.strip_suffix('?'), None, None, false, false);
            }) as Box<dyn FnMut(_)>,
        ),
        ListObject::UserGroup => (
            message.unwrap_or("User group name ['?' for list]"),
            Box::new(|o: String| {
                let _ = &list_usergroup(pgclient, o.strip_suffix('?'), None, false, false, false);
            }) as Box<dyn FnMut(_)>,
        ),
        ListObject::KeyID => (
            message.unwrap_or("Key ID ['?' for list]"),
            Box::new(|o: String| {
                let _ = &list_key(pgclient, o.strip_suffix('?'), None, false, None, false);
            }) as Box<dyn FnMut(_)>,
        ),
        ListObject::ServerGroup => (
            message.unwrap_or("Server group name ['?' for list]"),
            Box::new(|o: String| {
                let _ = &list_servergroup(
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
                let _ = &list_serveraccess(
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

    loop {
        let o = set_or_ask_for(key, prompt);
        if o.trim().ends_with('?') {
            let _ = &lcmd(o);
        } else {
            return o;
        }
    }
}

fn gen_ssh_fingerprint(key: &str) -> String {
    let sshkey = key.split(' ').collect::<Vec<&str>>();

    if sshkey.len() < 2 {
        exit_with_message("Wrong SSH key format.");
    }

    if sshkey[0].eq("ssh-ed25519") || sshkey[0].eq("ssh-rsa") || sshkey[0].eq("ecdsa-sha2-nistp256")
    {
        if let Ok(binkey) = decode(sshkey[1]) {
            let mut hasher = Sha256::new();
            hasher.update(binkey);
            let result = hasher.finalize();
            let mut fingerprint = String::from("SHA256:");
            fingerprint.push_str(&encode(result));
            return fingerprint.trim_end_matches('=').to_string();
        }
    }

    exit_with_message("Wrong SSH key format.");
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

#[allow(clippy::too_many_lines)]
fn main() {
    let matches = cli::build_cli().get_matches();
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
            &config
                .db
                .path
                .unwrap_or_else(|| "/var/run/postgres".to_string()),
        )
        .host(&config.db.host.unwrap_or_else(|| "localhost".to_string()))
        .dbname(&config.db.dbname.unwrap_or_else(|| "oere".to_string()));

    if let Some(password) = config.client.password {
        pgconfig.password(&password);
    }

    let mut con = match pgconfig.connect(NoTls) {
        Ok(con) => con,
        Err(_) => {
            exit_with_message("Could not connect to database.");
        }
    };

    // logging
    let logfile = if let Some(logconfig) = config.logging {
        logconfig
            .file
            .unwrap_or_else(|| PathBuf::from(DEFAULT_LOGFILE))
    } else {
        PathBuf::from(DEFAULT_LOGFILE)
    };

    if let Err(e) = create_logger(&logfile) {
        error!("Could create logger: {}", e);
        std::process::exit(1);
    }

    if let Some(submatches) = matches.subcommand_matches("list-user") {
        if list_user(
            &mut con,
            submatches
                .get_one::<String>("EMAIL")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("ID")
                .map(std::string::String::as_str),
            submatches.contains_id("EXACT"),
            matches.contains_id("JSONOUTPUT"),
        )
        .is_err()
        {
            exit_with_message("Could not list users.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("list-key") {
        if list_key(
            &mut con,
            submatches
                .get_one::<String>("EMAIL")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("FINGERPRINT")
                .map(std::string::String::as_str),
            submatches.contains_id("WITHKEY"),
            submatches
                .get_one::<String>("ID")
                .map(std::string::String::as_str),
            matches.contains_id("JSONOUTPUT"),
        )
        .is_err()
        {
            exit_with_message("Could not list keys.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("list-server") {
        if list_server(
            &mut con,
            submatches
                .get_one::<String>("SERVERNAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("IP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("ID")
                .map(std::string::String::as_str),
            submatches.contains_id("EXACT"),
            matches.contains_id("JSONOUTPUT"),
        )
        .is_err()
        {
            exit_with_message("Could not list server.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("list-serverauth") {
        if list_serverauth(
            &mut con,
            submatches
                .get_one::<String>("IP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVERNAME")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not list server auth.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("write-serverauth") {
        write_serverauth(
            &mut con,
            submatches
                .get_one::<OsString>("WORKDIR")
                .map(std::ffi::OsString::as_os_str),
            submatches.contains_id("FORCE"),
        );
    }

    if let Some(submatches) = matches.subcommand_matches("list-servergroup") {
        if list_servergroup(
            &mut con,
            submatches
                .get_one::<String>("SERVERGROUP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVERNAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("IP")
                .map(std::string::String::as_str),
            submatches.contains_id("EXACT"),
            submatches.contains_id("ALL"),
            submatches.contains_id("EMPTY"),
            matches.contains_id("JSONOUTPUT"),
        )
        .is_err()
        {
            exit_with_message("Could not list server groups.");
        };
    }
    if let Some(submatches) = matches.subcommand_matches("list-usergroup") {
        if list_usergroup(
            &mut con,
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("EMAIL")
                .map(std::string::String::as_str),
            submatches.contains_id("EXACT"),
            submatches.contains_id("EMPTY"),
            matches.contains_id("JSONOUTPUT"),
        )
        .is_err()
        {
            exit_with_message("Could not list user groups.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("list-useraccess") {
        if list_useraccess(
            &mut con,
            submatches
                .get_one::<String>("EMAIL")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVERNAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("IP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("USER")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVERACCESS")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVERGROUP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("USERGROUP")
                .map(std::string::String::as_str),
            submatches.contains_id("EXACT"),
            submatches.contains_id("EXPIRED"),
            matches.contains_id("JSONOUTPUT"),
        )
        .is_err()
        {
            exit_with_message("Could not list user access.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("list-serveraccess") {
        if list_serveraccess(
            &mut con,
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVERNAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("IP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SSHUSER")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVERGROUP")
                .map(std::string::String::as_str),
            submatches.contains_id("EXACT"),
            matches.contains_id("JSONOUTPUT"),
        )
        .is_err()
        {
            exit_with_message("Could not list server access.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("add-server") {
        if add_server(
            &mut con,
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("IP")
                .map(std::string::String::as_str),
            submatches.contains_id("DISABLED"),
            submatches.contains_id("DNS"),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not add server.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("add-user") {
        if add_user(
            &mut con,
            submatches
                .get_one::<String>("EMAIL")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("TYPE")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not add user.");
        }
    }

    if let Some(submatches) = matches.subcommand_matches("add-key") {
        if add_key(
            &mut con,
            submatches
                .get_one::<String>("EMAIL")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("KEY")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not add key.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("add-usergroup") {
        if add_usergroup(
            &mut con,
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not add user group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("add-user-to-usergroup") {
        if add_user_to_usergroup(
            &mut con,
            submatches
                .get_one::<String>("EMAIL")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not add user to user group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("add-usergroup-to-usergroup") {
        if add_usergroup_to_usergroup(
            &mut con,
            submatches
                .get_one::<String>("SUBGROUP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SUPERGROUP")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not add user group to user group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("add-servergroup") {
        if add_servergroup(
            &mut con,
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not add server group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("add-server-to-servergroup") {
        if add_server_to_servergroup(
            &mut con,
            submatches
                .get_one::<String>("SERVER")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not add server to server group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("add-servergroup-to-servergroup") {
        if add_servergroup_to_servergroup(
            &mut con,
            submatches
                .get_one::<String>("SUBGROUP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SUPERGROUP")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not add server group to server group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("add-useraccess") {
        if add_useraccess(
            &mut con,
            submatches
                .get_one::<String>("EMAIL")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("USERGROUP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVERACCESS")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("UNTIL")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not add user access.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("add-serveraccess") {
        if add_serveraccess(
            &mut con,
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SSHUSER")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SSHFROM")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SSHCOMMAND")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SSHOPTION")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVER")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVERGROUP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not add server access.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("delete-server") {
        if delete_server(
            &mut con,
            submatches
                .get_one::<String>("SERVER")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not delete server.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("delete-user") {
        if delete_user(
            &mut con,
            submatches
                .get_one::<String>("EMAIL")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not delete user.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("delete-key") {
        if delete_key(
            &mut con,
            submatches
                .get_one::<String>("KEYID")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not delete key.");
        };
    }
    if let Some(submatches) = matches.subcommand_matches("delete-user-from-usergroup") {
        if delete_user_from_usergroup(
            &mut con,
            submatches
                .get_one::<String>("EMAIL")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("USERGROUP")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not delete user from user group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("delete-usergroup-from-usergroup") {
        if delete_usergroup_from_usergroup(
            &mut con,
            submatches
                .get_one::<String>("SUBGROUP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SUPERGROUP")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not delete user group from user group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("delete-server-from-servergroup") {
        if delete_server_from_servergroup(
            &mut con,
            submatches
                .get_one::<String>("SERVER")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVERGROUP")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not delete server from server group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("delete-servergroup-from-servergroup") {
        if delete_servergroup_from_servergroup(
            &mut con,
            submatches
                .get_one::<String>("SUBGROUP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SUPERGROUP")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not delete server group from server group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("delete-usergroup") {
        if delete_usergroup(
            &mut con,
            submatches
                .get_one::<String>("USERGROUP")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not delete user group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("delete-servergroup") {
        if delete_servergroup(
            &mut con,
            submatches
                .get_one::<String>("SERVERGROUP")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not delete server group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("delete-serveraccess") {
        if delete_serveraccess(
            &mut con,
            submatches
                .get_one::<String>("SERVERACCESS")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not delete server access.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("delete-useraccess") {
        if delete_useraccess(
            &mut con,
            submatches
                .get_one::<String>("EMAIL")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("USERGROUP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVERACCESS")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not delete user access.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("disable-user") {
        if disable_user(
            &mut con,
            submatches
                .get_one::<String>("USEREMAIL")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not disable user.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("enable-user") {
        if enable_user(
            &mut con,
            submatches
                .get_one::<String>("USEREMAIL")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not enable user.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("disable-server") {
        if disable_server(
            &mut con,
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not disable server.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("enable-server") {
        if enable_server(
            &mut con,
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not enable server.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("disable-dns") {
        if disable_dns(
            &mut con,
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not disable server DNS lookup.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("enable-dns") {
        if enable_dns(
            &mut con,
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches.contains_id("FORCE"),
        )
        .is_err()
        {
            exit_with_message("Could not enable server DNS lookup.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("update-server") {
        if update_server(
            &mut con,
            submatches
                .get_one::<String>("SERVERID")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("IP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not update server.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("update-servergroup") {
        if update_servergroup(
            &mut con,
            submatches
                .get_one::<String>("SERVERGROUP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("NEWNAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not update server group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("update-user") {
        if update_user(
            &mut con,
            submatches
                .get_one::<String>("USERID")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("EMAIL")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("TYPE")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not update user.");
        };
    }
    if let Some(submatches) = matches.subcommand_matches("update-usergroup") {
        if update_usergroup(
            &mut con,
            submatches
                .get_one::<String>("USERGROUP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("NEWNAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not update user group.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("update-key") {
        if update_key(
            &mut con,
            submatches
                .get_one::<String>("KEYID")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("KEY")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not update SSH key.");
        };
    }

    if let Some(submatches) = matches.subcommand_matches("update-serveraccess") {
        if update_serveraccess(
            &mut con,
            submatches
                .get_one::<String>("NAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("NEWNAME")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SSHUSER")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SSHFROM")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SSHCOMMAND")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SSHOPTION")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVER")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("SERVERGROUP")
                .map(std::string::String::as_str),
            submatches
                .get_one::<String>("COMMENT")
                .map(std::string::String::as_str),
        )
        .is_err()
        {
            exit_with_message("Could not update server access.");
        };
    }
}
