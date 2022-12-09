#![allow(clippy::module_name_repetitions)]
use log::info;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, Error};
use std::io::{self, Write};
use std::net::IpAddr;
use time::{format_description, Date, PrimitiveDateTime, Time};

use crate::{ask_for, exit_with_message, gen_ssh_fingerprint, set_or_ask_for, ListObject};

use crate::logging::get_ssh_client;

use crate::schema::Usertype;

pub fn add_server(
    pgclient: &mut Client,
    servername: Option<&str>,
    ip: Option<&str>,
    disabled: bool,
    use_dns: bool,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Add server");
    let query_string = r#"INSERT INTO server (name, ip, disabled, use_dns, comment)
                          VALUES ($1, $2, $3, $4, $5)"#;

    let newservername = ask_for(&ListObject::ServerName, servername, None, pgclient);

    if newservername.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM server WHERE name = $1 LIMIT 1"#,
        &[&newservername],
    )?;

    if !res.is_empty() {
        exit_with_message("Name already in use.");
    }

    let newip = set_or_ask_for(ip, "IP");

    let newip = match newip.parse::<IpAddr>() {
        Ok(address) => address,
        Err(_) => exit_with_message("This is not a valid IP address'"),
    };

    let res = pgclient.query(r#"SELECT id FROM server WHERE ip = $1 LIMIT 1"#, &[&newip])?;

    if !res.is_empty() {
        exit_with_message("IP already in use.");
    }

    let newcomment = set_or_ask_for(comment, "Comment");
    let newcomment = match &newcomment.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newcomment),
    };

    pgclient.query(
        query_string,
        &[&newservername, &newip, &disabled, &use_dns, &newcomment],
    )?;

    info!(
        "({}) Added server '{}' ('{}')",
        &get_ssh_client(),
        &newservername,
        &newip
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn add_user(
    pgclient: &mut Client,
    email: Option<&str>,
    username: Option<&str>,
    usertype: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Add user");
    let query_string = r#"INSERT INTO "user" (email, name, type, comment)
                          VALUES ($1, $2, $3, $4)"#;

    let newemail = ask_for(&ListObject::UserEmail, email, None, pgclient);

    if newemail.eq("") {
        exit_with_message("User email cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM "user" WHERE email = $1 LIMIT 1"#,
        &[&newemail],
    )?;

    if !res.is_empty() {
        exit_with_message("Email already in use.");
    }

    let newname = set_or_ask_for(username, "Name");

    if newname.eq("") {
        exit_with_message("User name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM "user" WHERE name = $1 LIMIT 1"#,
        &[&newname],
    )?;

    if !res.is_empty() {
        exit_with_message("User name already in use.");
    }

    let usertype_prompt = set_or_ask_for(usertype, "Type [AD user/tool user/external user]");

    let newutype = match usertype_prompt.trim().to_lowercase().as_str() {
        "ad" | "ad user" | "" => Usertype::AD,
        "tool" | "tool user" => Usertype::Tool,
        "external" | "external user" => Usertype::External,
        _ => exit_with_message("Invalid user type."),
    };

    let newcomment = set_or_ask_for(comment, "Comment");
    let newcomment = match &newcomment.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newcomment),
    };

    pgclient.query(query_string, &[&newemail, &newname, &newutype, &newcomment])?;

    info!(
        "({}) Added user '{}' ('{}')",
        &get_ssh_client(),
        &newemail,
        &newname
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    println!();
    print!("Do you want to add a SSH key for this user? [Y/n]: ");
    let mut userinput = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut userinput).unwrap();
    if !userinput.trim().to_lowercase().eq("n") {
        add_key(pgclient, Some(&newemail), None, None)?;
    }

    Ok(())
}

pub fn add_key(
    pgclient: &mut Client,
    email: Option<&str>,
    publickey: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Add SSH key");
    let query_string = r#"INSERT INTO sshkeys (user_id, sshkey, fingerprint, comment)
                          SELECT "user".id,
                                 $1,
                                 $2,
                                 $3
                          FROM "user"
                          WHERE "user".email = $4"#;

    let newemail = ask_for(&ListObject::UserEmail, email, None, pgclient);

    if newemail.eq("") {
        exit_with_message("User email cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM "user" WHERE email = $1 LIMIT 1"#,
        &[&newemail],
    )?;

    if res.is_empty() {
        exit_with_message("User not found.")
    }

    let mut newkey = set_or_ask_for(publickey, "Public SSH key");

    if newkey.eq("") {
        exit_with_message("Key cannot be empty.");
    }

    if newkey.split(' ').count() < 2 {
        exit_with_message("Invalid key format.")
    }

    newkey = newkey.split(' ').collect::<Vec<&str>>()[..2].join(" ");
    let fingerprint = &gen_ssh_fingerprint(&newkey);

    let newcomment = set_or_ask_for(comment, "Comment");
    let newcomment = match &newcomment.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newcomment),
    };

    pgclient.query(
        query_string,
        &[&newkey, &fingerprint, &newcomment, &newemail],
    )?;

    info!(
        "({}) Added SSH key '{}' for user '{}'",
        &get_ssh_client(),
        &fingerprint,
        &newemail
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn add_usergroup(
    pgclient: &mut Client,
    usergroup: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Add user group");
    let query_string = r#"INSERT INTO usergroup (name, comment) VALUES ($1, $2)"#;

    let newname = ask_for(&ListObject::UserGroup, usergroup, None, pgclient);

    if newname.eq("") {
        exit_with_message("Group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM usergroup WHERE name = $1 LIMIT 1"#,
        &[&newname],
    )?;

    if !res.is_empty() {
        exit_with_message("Group name already in use.");
    }

    let newcomment = set_or_ask_for(comment, "Comment");
    let newcomment = match &newcomment.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newcomment),
    };

    pgclient.query(query_string, &[&newname, &newcomment])?;

    info!("({}) Added user group '{}'", &get_ssh_client(), &newname);

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    if usergroup.is_none() && comment.is_none() {
        loop {
            println!();
            print!("Do you want to add a user to this user group? [Y/n]: ");
            let mut userinput = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut userinput).unwrap();
            if userinput.trim().to_lowercase().eq("n") {
                break;
            }
            add_user_to_usergroup(pgclient, None, Some(&newname))?;
        }
    }

    Ok(())
}

pub fn add_user_to_usergroup(
    pgclient: &mut Client,
    email: Option<&str>,
    usergroup: Option<&str>,
) -> Result<(), Error> {
    println!("Add user to user group");
    let query_string = r#"INSERT INTO user_usergroup (user_id, usergroup_id)
                          SELECT "user".id,
                                 usergroup.id
                          FROM "user"
                          JOIN usergroup ON usergroup.name = $1
                          WHERE "user".email = $2"#;

    let newname = ask_for(&ListObject::UserGroup, usergroup, None, pgclient);

    if newname.eq("") {
        exit_with_message("User group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM usergroup WHERE name = $1 LIMIT 1"#,
        &[&newname],
    )?;

    if res.is_empty() {
        exit_with_message("Group not found.");
    }

    let newemail = ask_for(&ListObject::UserEmail, email, None, pgclient);

    if newemail.eq("") {
        exit_with_message("User email cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM "user" WHERE email = $1 LIMIT 1"#,
        &[&newemail],
    )?;

    if res.is_empty() {
        exit_with_message("User not found.");
    }

    let res = pgclient.query(
        r#"SELECT user_id
               FROM user_usergroup
               JOIN "user" ON "user".id = user_usergroup.user_id
               JOIN usergroup ON usergroup.id = user_usergroup.usergroup_id
               WHERE "user".email = $1
                 AND usergroup.name = $2"#,
        &[&newemail, &newname],
    )?;

    if res.is_empty() {
        pgclient.query(query_string, &[&newname, &newemail])?;
    } else {
        println!();
        println!(
            "{} User already in user group.",
            "warning:".if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
        );
    }

    info!(
        "({}) Added user '{}' to user group '{}'",
        &get_ssh_client(),
        &newemail,
        &newname
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    if email.is_none() && usergroup.is_none() {
        loop {
            println!();
            print!("Do you want to add another user to this user group? [Y/n]: ");
            let mut userinput = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut userinput).unwrap();
            if userinput.trim().to_lowercase().eq("n") {
                std::process::exit(0);
            }
            add_user_to_usergroup(pgclient, None, Some(&newname))?;
        }
    }

    Ok(())
}

pub fn add_usergroup_to_usergroup(
    pgclient: &mut Client,
    subgroup: Option<&str>,
    supergroup: Option<&str>,
) -> Result<(), Error> {
    println!("Add user group to user group");
    let query_string = r#"INSERT INTO usergroup_usergroup (subgroup_id, supergroup_id)
                          SELECT ug1.id,
                            (SELECT ug2.id
                             FROM usergroup ug2
                             WHERE ug2.name = $2)
                          FROM usergroup ug1
                          WHERE ug1.name = $1"#;

    let newsubgroupname = ask_for(
        &ListObject::UserGroup,
        subgroup,
        Some("(Member) group name ['?' for list]"),
        pgclient,
    );

    if newsubgroupname.eq("") {
        exit_with_message("Group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM usergroup WHERE name = $1 LIMIT 1"#,
        &[&newsubgroupname],
    )?;

    if res.is_empty() {
        exit_with_message("User group not found.");
    }

    let newsupergroupname = ask_for(
        &ListObject::UserGroup,
        supergroup,
        Some("(Parent) group name ['?' for list]"),
        pgclient,
    );

    if newsupergroupname.eq("") {
        exit_with_message("Group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM usergroup WHERE name = $1 LIMIT 1"#,
        &[&newsupergroupname],
    )?;

    if res.is_empty() {
        exit_with_message("User group not found.");
    }

    let res = pgclient.query(
        r#"SELECT subgroup_id, supergroup_id
           FROM usergroup_usergroup
           JOIN usergroup AS ug1 ON ug1.id = supergroup_id
           JOIN usergroup AS ug2 ON ug2.id = subgroup_id
           WHERE ug1.name = $1
             AND ug2.name = $2"#,
        &[&newsupergroupname, &newsubgroupname],
    )?;

    if res.is_empty() {
        pgclient.query(query_string, &[&newsubgroupname, &newsupergroupname])?;
    } else {
        println!();
        println!(
            "{} User group already in user group.",
            "warning:".if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
        );
    }

    info!(
        "({}) Added user group '{}' to user group '{}'",
        &get_ssh_client(),
        &newsubgroupname,
        &newsupergroupname
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    if subgroup.is_none() && supergroup.is_none() {
        loop {
            println!();
            print!("Do you want to add another user group to this user group? [Y/n]: ");
            let mut userinput = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut userinput).unwrap();
            if userinput.trim().to_lowercase().eq("n") {
                break;
            }
            add_usergroup_to_usergroup(pgclient, None, Some(&newsupergroupname))?;
        }
    }

    Ok(())
}

pub fn add_servergroup(
    pgclient: &mut Client,
    servergroup: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Add server group");
    let query_string = r#"INSERT INTO servergroup (name, comment) VALUES ($1, $2)"#;

    let newname = ask_for(&ListObject::ServerGroup, servergroup, None, pgclient);

    if newname.eq("") {
        exit_with_message("Server group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM servergroup WHERE name = $1 LIMIT 1"#,
        &[&newname],
    )?;

    if !res.is_empty() {
        exit_with_message("Group name already in use.");
    }

    let newcomment = set_or_ask_for(comment, "Comment");
    let newcomment = match &newcomment.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newcomment),
    };

    pgclient.query(query_string, &[&newname, &newcomment])?;

    info!("({}) Added server group '{}'", &get_ssh_client(), &newname);

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    if servergroup.is_none() && comment.is_none() {
        loop {
            println!();
            print!("Do you want to add a server to this server group? [Y/n]: ");
            let mut userinput = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut userinput).unwrap();
            if userinput.trim().to_lowercase().eq("n") {
                break;
            }
            add_server_to_servergroup(pgclient, None, Some(&newname))?;
        }
    }

    Ok(())
}

pub fn add_server_to_servergroup(
    pgclient: &mut Client,
    servername: Option<&str>,
    servergroup: Option<&str>,
) -> Result<(), Error> {
    println!("Add server to server group");
    let query_string = r#"INSERT INTO server_servergroup (server_id, servergroup_id)
                          SELECT server.id, servergroup.id
                          FROM server
                          JOIN servergroup
                          ON servergroup.name = $1
                          WHERE server.name = $2"#;

    let newgroupname = ask_for(&ListObject::ServerGroup, servergroup, None, pgclient);

    if newgroupname.eq("") {
        exit_with_message("Name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM servergroup WHERE name = $1 LIMIT 1"#,
        &[&newgroupname],
    )?;

    if res.is_empty() {
        exit_with_message("Group not found.");
    }

    let newservername = ask_for(&ListObject::ServerName, servername, None, pgclient);

    if newservername.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM server WHERE name = $1 LIMIT 1"#,
        &[&newservername],
    )?;

    if res.is_empty() {
        exit_with_message("Server not found.");
    }

    let res = pgclient.query(
        r#"SELECT server_id FROM server_servergroup
           JOIN server ON server.id = server_servergroup.server_id
           JOIN servergroup ON servergroup.id = server_servergroup.servergroup_id
            WHERE server.name = $1
              AND servergroup.name = $2"#,
        &[&newservername, &newgroupname],
    )?;

    if res.is_empty() {
        pgclient.query(query_string, &[&newgroupname, &newservername])?;
    } else {
        println!();
        println!(
            "{} Server already in server group.",
            "warning:".if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
        );
    }

    info!(
        "({}) Added server '{}' to server group '{}'",
        &get_ssh_client(),
        &newservername,
        &newgroupname
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    if servername.is_none() && servergroup.is_none() {
        loop {
            println!();
            print!("Do you want to add another server to this server group? [Y/n]: ");
            let mut userinput = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut userinput).unwrap();
            if userinput.trim().to_lowercase().eq("n") {
                break;
            }
            add_server_to_servergroup(pgclient, None, Some(&newgroupname))?;
        }
    }

    Ok(())
}

pub fn add_servergroup_to_servergroup(
    pgclient: &mut Client,
    subgroup: Option<&str>,
    supergroup: Option<&str>,
) -> Result<(), Error> {
    println!("Add server group to server group");
    let query_string = r#"INSERT INTO servergroup_servergroup (subgroup_id, supergroup_id)
                          SELECT sg1.id,
                            (SELECT sg2.id
                             FROM servergroup sg2
                             WHERE sg2.name = $2)
                          FROM servergroup sg1
                          WHERE sg1.name = $1"#;

    let newsubgroupname = ask_for(
        &ListObject::ServerGroup,
        subgroup,
        Some("(Member) group name ['?' for list]"),
        pgclient,
    );

    if newsubgroupname.eq("") {
        exit_with_message("Group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM servergroup WHERE name = $1 LIMIT 1"#,
        &[&newsubgroupname],
    )?;

    if res.is_empty() {
        exit_with_message("Server group not found.");
    }

    let newsupergroupname = ask_for(
        &ListObject::ServerGroup,
        supergroup,
        Some("(Parent) group name ['?' for list]"),
        pgclient,
    );

    if newsupergroupname.eq("") {
        exit_with_message("Group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM servergroup WHERE name = $1 LIMIT 1"#,
        &[&newsupergroupname],
    )?;

    if res.is_empty() {
        exit_with_message("Server group not found.");
    }

    let res = pgclient.query(
        r#"SELECT subgroup_id, supergroup_id
           FROM servergroup_servergroup
           JOIN servergroup AS sg1 ON sg1.id = supergroup_id
           JOIN servergroup AS sg2 ON sg2.id = subgroup_id
           WHERE sg1.name = $1
             AND sg2.name = $2"#,
        &[&newsupergroupname, &newsubgroupname],
    )?;

    if res.is_empty() {
        pgclient.query(query_string, &[&newsubgroupname, &newsupergroupname])?;
    } else {
        println!();
        println!(
            "{} Server group already in server group.",
            "warning:".if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
        );
    }

    info!(
        "({}) Added server group '{}' to server group '{}'",
        &get_ssh_client(),
        &newsubgroupname,
        &newsupergroupname
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    if subgroup.is_none() && supergroup.is_none() {
        loop {
            println!();
            print!("Do you want to add another server group to this server group? [Y/n]: ");
            let mut userinput = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut userinput).unwrap();
            if userinput.trim().to_lowercase().eq("n") {
                break;
            }
            add_servergroup_to_servergroup(pgclient, None, Some(&newsupergroupname))?;
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub fn add_serveraccess(
    pgclient: &mut Client,
    serveraccess: Option<&str>,
    sshuser: Option<&str>,
    sshfrom: Option<&str>,
    sshcommand: Option<&str>,
    sshoption: Option<&str>,
    servername: Option<&str>,
    servergroup: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Add server access");
    let query_string;

    let newname = ask_for(&ListObject::ServerAccess, serveraccess, None, pgclient);

    if newname.eq("") {
        exit_with_message("Server access name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM serveraccess WHERE name = $1 LIMIT 1"#,
        &[&newname],
    )?;

    if !res.is_empty() {
        exit_with_message("Name already in use.");
    }

    let mut newuser = set_or_ask_for(sshuser, "SSH user [default: administrator]");

    if newuser.eq("") {
        newuser = "administrator".to_string();
    }

    let newfrom = set_or_ask_for(sshfrom, "sshfrom");
    let newfrom = match &newfrom.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newfrom),
    };

    let newcommand = set_or_ask_for(sshcommand, "sshcommand");
    let newcommand = match &newcommand.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newcommand),
    };

    let newoption = set_or_ask_for(sshoption, "sshoption");
    let newoption = match &newoption.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newoption),
    };

    let newservername = if servergroup.is_none() {
        ask_for(&ListObject::ServerName, servername, None, pgclient)
    } else {
        String::new()
    };

    let newservername = match &newservername.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newservername),
    };

    let newservername = if newservername.is_none() {
        let newgroupname = ask_for(&ListObject::ServerGroup, servergroup, None, pgclient);

        match &newgroupname.trim().to_lowercase()[..] {
            "" | "null" => {
                exit_with_message("Server name and server group name cannot *both* be empty.")
            }
            _ => {
                let res = pgclient.query(
                    r#"SELECT id FROM servergroup WHERE name = $1 LIMIT 1"#,
                    &[&newgroupname],
                )?;

                if res.is_empty() {
                    exit_with_message("Server group not found.");
                }

                query_string = r#"INSERT INTO serveraccess (name, sshuser, sshfrom, sshcommand, sshoption, comment, servergroup_id)
                                  SELECT $1, $2, $3, $4, $5, $6, id
                                  FROM servergroup
                                  WHERE servergroup.name = $7"#.to_string();
                Some(newgroupname)
            }
        }
    } else {
        let res = pgclient.query(
            r#"SELECT id FROM server WHERE name = $1 LIMIT 1"#,
            &[&newservername],
        )?;

        if res.is_empty() {
            exit_with_message("Server not found.");
        }

        query_string = r#"INSERT INTO serveraccess (name, sshuser, sshfrom, sshcommand, sshoption, comment, server_id)
                          SELECT $1, $2, $3, $4, $5, $6, id
                          FROM server
                          WHERE server.name = $7"#.to_string();
        newservername
    };

    let newcomment = set_or_ask_for(comment, "Comment");
    let newcomment = match &newcomment.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newcomment),
    };

    pgclient.query(
        &query_string,
        &[
            &newname,
            &newuser,
            &newfrom,
            &newcommand,
            &newoption,
            &newcomment,
            &newservername,
        ],
    )?;

    info!("({}) Added server access '{}'", &get_ssh_client(), &newname);

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn add_useraccess(
    pgclient: &mut Client,
    email: Option<&str>,
    usergroup: Option<&str>,
    serveraccess: Option<&str>,
    until: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Add user access");
    let query_string;

    let newemail = if usergroup.is_none() {
        ask_for(&ListObject::UserEmail, email, None, pgclient)
    } else {
        String::new()
    };

    let newemail = match &newemail.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newemail),
    };

    let newemail = if newemail.is_none() {
        let newgroupname = ask_for(&ListObject::UserGroup, usergroup, None, pgclient);

        match &newgroupname.trim().to_lowercase()[..] {
            "" | "null" => {
                exit_with_message("User email and user group name cannot *both* be empty.")
            }
            _ => {
                let res = pgclient.query(
                    r#"SELECT id FROM usergroup WHERE name = $1 LIMIT 1"#,
                    &[&newgroupname],
                )?;

                if res.is_empty() {
                    exit_with_message("User group not found.");
                }

                query_string =
                    r#"INSERT INTO useraccess (usergroup_id, serveraccess_id, comment, best_before)
                                  SELECT usergroup.id, serveraccess.id, $1, $2
                                  FROM usergroup,serveraccess
                                  WHERE usergroup.name = $3
                                    AND serveraccess.name = $4"#
                        .to_string();
                Some(newgroupname)
            }
        }
    } else {
        let res = pgclient.query(
            r#"SELECT id FROM "user" WHERE email = $1 LIMIT 1"#,
            &[&newemail],
        )?;

        if res.is_empty() {
            exit_with_message("User not found.");
        }

        query_string = r#"INSERT INTO useraccess (user_id, serveraccess_id, comment, best_before)
                          SELECT "user".id, serveraccess.id, $1, $2
                          FROM "user",serveraccess
                          WHERE "user".email = $3
                            AND serveraccess.name = $4"#
            .to_string();
        newemail
    };

    let newname = ask_for(&ListObject::ServerAccess, serveraccess, None, pgclient);

    if newname.eq("") {
        exit_with_message("Server access name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM serveraccess WHERE name = $1"#,
        &[&newname],
    )?;

    if res.is_empty() {
        exit_with_message("Server access not found.");
    }

    let newuntil = set_or_ask_for(until, "until");
    let newuntil = match &newuntil.trim().to_lowercase()[..] {
        "" | "null" => PrimitiveDateTime::new(
            Date::parse(
                "2256-05-11",
                &format_description::parse("[year]-[month]-[day]")
                    .expect("BUG: DateTimeFormatDesc"),
            )
            .expect("BUG: Default DateTime"),
            Time::MIDNIGHT,
        ),
        _ => {
            if newuntil.split(' ').count() == 1 {
                let date = match Date::parse(
                    newuntil.split(' ').collect::<Vec<&str>>()[0],
                    &format_description::parse("[year]-[month]-[day]")
                        .expect("BUG: DateTimeFormatDesc"),
                ) {
                    Ok(d) => d,
                    _ => exit_with_message("Could not parse date."),
                };
                PrimitiveDateTime::new(date, Time::MIDNIGHT)
            } else if newuntil.split(' ').count() == 2 {
                let date = match Date::parse(
                    newuntil.split(' ').collect::<Vec<&str>>()[0],
                    &format_description::parse("[year]-[month]-[day]")
                        .expect("BUG: DateTimeFormatDesc"),
                ) {
                    Ok(d) => d,
                    _ => exit_with_message("Could not parse date."),
                };
                let time = match Time::parse(
                    newuntil.split(' ').collect::<Vec<&str>>()[1],
                    &format_description::parse("[hour]:[minute]:[second]")
                        .expect("BUG: DateTimeFormatDesc"),
                ) {
                    Ok(t) => t,
                    _ => exit_with_message("Could not parse time."),
                };
                PrimitiveDateTime::new(date, time)
            } else {
                exit_with_message("Could not parse datetime.");
            }
        }
    };

    let newcomment = set_or_ask_for(comment, "Comment");
    let newcomment = match &newcomment.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newcomment),
    };

    pgclient.query(
        &query_string,
        &[&newcomment, &newuntil, &newemail, &newname],
    )?;

    info!(
        "({}) Added user access '{}' for '{}'",
        &get_ssh_client(),
        &newname,
        &newemail.unwrap_or_else(|| "-".to_string())
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}
