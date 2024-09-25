use crate::commands;

mod key;
mod server;
mod serveraccess;
mod serverauth;
mod servergroup;
mod user;
mod useraccess;
mod usergroup;

const HELP: &str = "oerec
\u{d8}re client

Usage: oerec [OPTIONS] [COMMAND]

Options:
    -h, --help       Print this message or the help of the given subcommand
    -V, --version    Print version information

Commands:
    add-server, list-server, update-server, delete-server

    add-servergroup, list-servergroup, update-servergroup, delete-servergroup

    add-server-to-servergroup, add-servergroup-to-servergroup
    delete-server-from-servergroup, delete-servergroup-from-servergroup

    add-user, list-user, update-user, delete-user

    add-key, list-key, update-key, delete-key

    add-usergroup, list-usergroup, update-usergroup, delete-usergroup

    add-user-to-usergroup, add-usergroup-to-usergroup
    delete-user-from-usergroup, delete-usergroup-from-usergroup

    add-serveraccess, list-serveraccess, update-serveraccess, delete-serveraccess
    add-useraccess, list-useraccess, delete-useraccess

    enable-dns, disable-dns
    enable-server, disable-server
    enable-user, disable-user

    write-serverauth";

#[allow(clippy::too_many_lines)]
pub fn parse_subcommands(con: &mut postgres::Client) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('V') | Long("version") => {
                println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            Short('h') | Long("help") => {
                println!("{HELP}");
                std::process::exit(0);
            }
            Value(value) => {
                let value = value.string()?;
                match value.as_str() {
                    "list-key" | "list-keys" => {
                        commands::key::list(con, &mut parser)?;
                    }
                    "list-server" | "list-servers" => {
                        commands::server::list(con, &mut parser)?;
                    }
                    "list-serveraccess" => {
                        commands::serveraccess::list(con, &mut parser)?;
                    }
                    "list-serverauth" => {
                        commands::serverauth::list(con, &mut parser)?;
                    }
                    "list-servergroup" | "list-servergroups" => {
                        commands::servergroup::list(con, &mut parser)?;
                    }
                    "list-user" | "list-users" => {
                        commands::user::list(con, &mut parser)?;
                    }
                    "list-useraccess" => {
                        commands::useraccess::list(con, &mut parser)?;
                    }
                    "list-usergroup" | "list-usergroups" => {
                        commands::usergroup::list(con, &mut parser)?;
                    }
                    "add-key" => {
                        commands::key::add(con, &mut parser)?;
                    }
                    "add-server" => {
                        commands::server::add(con, &mut parser)?;
                    }
                    "add-server-to-servergroup" => {
                        commands::server::add_to_servergroup(con, &mut parser)?;
                    }
                    "add-serveraccess" => {
                        commands::serveraccess::add(con, &mut parser)?;
                    }
                    "add-servergroup" => {
                        commands::servergroup::add(con, &mut parser)?;
                    }
                    "add-servergroup-to-servergroup" => {
                        commands::servergroup::add_to_servergroup(con, &mut parser)?;
                    }
                    "add-user" => {
                        commands::user::add(con, &mut parser)?;
                    }
                    "add-user-to-usergroup" => {
                        commands::user::add_to_usergroup(con, &mut parser)?;
                    }
                    "add-useraccess" => {
                        commands::useraccess::add(con, &mut parser)?;
                    }
                    "add-usergroup" => {
                        commands::usergroup::add(con, &mut parser)?;
                    }
                    "add-usergroup-to-usergroup" => {
                        commands::usergroup::add_to_usergroup(con, &mut parser)?;
                    }
                    "delete-key" => {
                        commands::key::delete(con, &mut parser)?;
                    }
                    "delete-server" => {
                        commands::server::delete(con, &mut parser)?;
                    }
                    "delete-server-from-servergroup" => {
                        commands::server::delete_from_servergroup(con, &mut parser)?;
                    }
                    "delete-serveraccess" => {
                        commands::serveraccess::delete(con, &mut parser)?;
                    }
                    "delete-servergroup" => {
                        commands::servergroup::delete(con, &mut parser)?;
                    }
                    "delete-servergroup-from-servergroup" => {
                        commands::servergroup::delete_from_servergroup(con, &mut parser)?;
                    }
                    "delete-user" => {
                        commands::user::delete(con, &mut parser)?;
                    }
                    "delete-user-from-usergroup" => {
                        commands::user::delete_from_usergroup(con, &mut parser)?;
                    }
                    "delete-useraccess" => {
                        commands::useraccess::delete(con, &mut parser)?;
                    }
                    "delete-usergroup" => {
                        commands::usergroup::delete(con, &mut parser)?;
                    }
                    "delete-usergroup-from-usergroup" => {
                        commands::usergroup::delete_from_usergroup(con, &mut parser)?;
                    }
                    "update-key" => {
                        commands::key::update(con, &mut parser)?;
                    }
                    "update-server" => {
                        commands::server::update(con, &mut parser)?;
                    }
                    "update-serveraccess" => {
                        commands::serveraccess::update(con, &mut parser)?;
                    }
                    "update-servergroup" => {
                        commands::servergroup::update(con, &mut parser)?;
                    }
                    "update-user" => {
                        commands::user::update(con, &mut parser)?;
                    }
                    "update-usergroup" => {
                        commands::usergroup::update(con, &mut parser)?;
                    }
                    "enable-dns" => {
                        commands::server::enable_dns(con, &mut parser)?;
                    }
                    "disable-dns" => {
                        commands::server::disable_dns(con, &mut parser)?;
                    }
                    "enable-server" => {
                        commands::server::enable(con, &mut parser)?;
                    }
                    "disable-server" => {
                        commands::server::disable(con, &mut parser)?;
                    }
                    "enable-user" => {
                        commands::user::enable(con, &mut parser)?;
                    }
                    "disable-user" => {
                        commands::user::disable(con, &mut parser)?;
                    }
                    "write-serverauth" => {
                        commands::serverauth::write(con, &mut parser)?;
                    }
                    value => {
                        return Err(format!("Unknown subcommand '{value}'").into());
                    }
                }
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(())
}
