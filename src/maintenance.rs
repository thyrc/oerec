#![allow(clippy::module_name_repetitions)]
use log::info;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, Error};
use std::io::{self, Write};

use crate::{ask_for, exit_with_message, ListObject};

use crate::logging::get_ssh_client;

pub fn disable_user(pgclient: &mut Client, email: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Disable user");
    let query_string = r#"UPDATE "user" SET disabled = true WHERE email = $1"#;

    let oldemail = ask_for(&ListObject::UserEmail, email, None, pgclient);

    if oldemail.eq("") {
        exit_with_message("User email cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT name FROM "user" WHERE email = $1 LIMIT 1"#,
        &[&oldemail],
    )?;

    if res.is_empty() {
        exit_with_message("User not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to disable user '{}'? [y/N]: ",
            &oldemail
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    let _ = pgclient.execute(query_string, &[&oldemail])?;

    info!("({}) Disabled user '{}'", &get_ssh_client(), &oldemail);

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn disable_server(pgclient: &mut Client, name: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Disable server");
    let query_string = r#"UPDATE server SET disabled = true WHERE name = $1"#;

    let oldname = ask_for(&ListObject::ServerName, name, None, pgclient);

    if oldname.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT name FROM server WHERE name = $1 LIMIT 1"#,
        &[&oldname],
    )?;

    if res.is_empty() {
        exit_with_message("Server not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to disable server '{}'? [y/N]: ",
            &oldname
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    let _ = pgclient.execute(query_string, &[&oldname])?;

    info!("({}) Disabled server '{}'", &get_ssh_client(), &oldname);

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn enable_user(pgclient: &mut Client, email: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Enable user");
    let query_string = r#"UPDATE "user" SET disabled = false WHERE email = $1"#;

    let oldemail = ask_for(&ListObject::UserEmail, email, None, pgclient);

    if oldemail.eq("") {
        exit_with_message("User email cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT name FROM "user" WHERE email = $1 LIMIT 1"#,
        &[&oldemail],
    )?;

    if res.is_empty() {
        exit_with_message("User not found.");
    }

    if !force {
        println!();
        print!("Do you really want to enable user '{}'? [y/N]: ", &oldemail);
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    let _ = pgclient.execute(query_string, &[&oldemail])?;

    info!("({}) Enabled user '{}'", &get_ssh_client(), &oldemail);

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn enable_server(pgclient: &mut Client, name: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Enable server");
    let query_string = r#"UPDATE server SET disabled = false WHERE name = $1"#;

    let oldname = ask_for(&ListObject::ServerName, name, None, pgclient);

    if oldname.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT name FROM server WHERE name = $1 LIMIT 1"#,
        &[&oldname],
    )?;

    if res.is_empty() {
        exit_with_message("Server not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to enable server '{}'? [y/N]: ",
            &oldname
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    let _ = pgclient.execute(query_string, &[&oldname])?;

    info!("({}) Enabled server '{}'", &get_ssh_client(), &oldname);

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn enable_dns(pgclient: &mut Client, name: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Enable server DNS lookup");
    let query_string = r#"UPDATE server SET use_dns = true WHERE name = $1"#;

    let oldname = ask_for(&ListObject::ServerName, name, None, pgclient);

    if oldname.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT name FROM server WHERE name = $1 LIMIT 1"#,
        &[&oldname],
    )?;

    if res.is_empty() {
        exit_with_message("Server not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to enable DNS lookup for server '{}'? [y/N]: ",
            &oldname
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    let _ = pgclient.execute(query_string, &[&oldname])?;

    info!(
        "({}) Enabled server DNS lookup '{}'",
        &get_ssh_client(),
        &oldname
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn disable_dns(pgclient: &mut Client, name: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Disable server DNS lookup");
    let query_string = r#"UPDATE server SET use_dns = false WHERE name = $1"#;

    let oldname = ask_for(&ListObject::ServerName, name, None, pgclient);

    if oldname.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT name FROM server WHERE name = $1 LIMIT 1"#,
        &[&oldname],
    )?;

    if res.is_empty() {
        exit_with_message("Server not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to disable DNS lookup for server '{}'? [y/N]: ",
            &oldname
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    let _ = pgclient.execute(query_string, &[&oldname])?;

    info!(
        "({}) Disabled server DNS lookup '{}'",
        &get_ssh_client(),
        &oldname
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}
