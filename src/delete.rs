#![allow(clippy::module_name_repetitions)]
use log::info;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, Error};
use std::io::{self, Write};

use crate::{ask_for, exit_with_message, ListObject};

use crate::logging::get_ssh_client;

pub fn delete_server(
    pgclient: &mut Client,
    servername: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Delete server");
    let query_string = r#"DELETE FROM server WHERE name = $1"#;

    let oldservername = ask_for(&ListObject::ServerName, servername, None, pgclient);

    if oldservername.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    if pgclient
        .query(
            r#"SELECT name FROM server WHERE name = $1 LIMIT 1"#,
            &[&oldservername],
        )?
        .is_empty()
    {
        exit_with_message("Server not found.");
    };

    if !force {
        println!();
        print!(
            "Do you really want to delete server '{}'? [y/N]: ",
            &oldservername
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    pgclient.query(query_string, &[&oldservername])?;

    info!(
        "({}) Deleted server '{}'",
        &get_ssh_client(),
        &oldservername
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn delete_user(pgclient: &mut Client, email: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Delete user");
    let query_string = r#"DELETE FROM "user" WHERE email = $1"#;

    let oldemail = ask_for(&ListObject::UserEmail, email, None, pgclient);

    if oldemail.eq("") {
        exit_with_message("User email cannot be empty.");
    }

    if pgclient
        .query(
            r#"SELECT name FROM "user" WHERE email = $1 LIMIT 1"#,
            &[&oldemail],
        )?
        .is_empty()
    {
        exit_with_message("User not found.");
    }

    if !force {
        println!();
        print!("Do you really want to delete user '{}'? [y/N]: ", &oldemail);
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    pgclient.query(query_string, &[&oldemail])?;

    info!("({}) Deleted user '{}'", &get_ssh_client(), &oldemail);

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn delete_key(pgclient: &mut Client, keyid: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Delete public SSH key");
    let query_string = r#"DELETE FROM sshkeys WHERE id = $1"#;

    let oldkeyid = ask_for(
        &ListObject::KeyID,
        keyid,
        Some("Key ID ['?' list by email"),
        pgclient,
    );

    if oldkeyid.eq("") {
        exit_with_message("Key ID cannot be empty.");
    }

    let keyint = match oldkeyid.parse::<i64>() {
        Ok(i) => i,
        Err(_) => exit_with_message("Wrong key ID format."),
    };

    let fingerprint = pgclient.query(
        r#"SELECT fingerprint FROM sshkeys WHERE id = $1 LIMIT 1"#,
        &[&keyint],
    )?;

    if fingerprint.is_empty() {
        exit_with_message("Key not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to delete key ID '{}' ({})? [y/N]: ",
            &oldkeyid,
            &fingerprint[0].get::<&str, String>("fingerprint")
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    pgclient.query(query_string, &[&keyint])?;

    info!(
        "({}) Deleted SSH key ID {} ({})",
        &get_ssh_client(),
        &keyint,
        &fingerprint[0].get::<&str, String>("fingerprint")
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn delete_user_from_usergroup(
    pgclient: &mut Client,
    email: Option<&str>,
    usergroup: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Delete user from user group");
    let query_string = r#"DELETE
                          FROM user_usergroup
                          WHERE user_usergroup =
                              (SELECT user_usergroup FROM user_usergroup
                               JOIN "user" ON user_usergroup.user_id = "user".id
                               JOIN usergroup ON user_usergroup.usergroup_id = usergroup.id
                               WHERE "user".email = $1
                                 AND usergroup.name = $2)"#;

    let oldemail = ask_for(&ListObject::UserEmail, email, None, pgclient);

    if oldemail.eq("") {
        exit_with_message("User email cannot be empty.");
    }

    if pgclient
        .query(
            r#"SELECT name FROM "user" WHERE email = $1 LIMIT 1"#,
            &[&oldemail],
        )?
        .is_empty()
    {
        exit_with_message("User not found.");
    }

    let oldusergroup = ask_for(&ListObject::UserGroup, usergroup, None, pgclient);

    if oldusergroup.eq("") {
        exit_with_message("Group name cannot be empty.");
    }

    if pgclient
        .query(
            r#"SELECT name FROM usergroup WHERE name = $1 LIMIT 1"#,
            &[&oldusergroup],
        )?
        .is_empty()
    {
        exit_with_message("User group not found.");
    }

    let singlegroup = pgclient.query(r#"SELECT usergroup.name
                                        FROM user_usergroup AS ug1
                                        JOIN
                                          (SELECT usergroup_id
                                           FROM user_usergroup
                                           GROUP BY usergroup_id
                                           HAVING COUNT(usergroup_id) = 1) AS ug2 ON ug1.usergroup_id = ug2.usergroup_id
                                        JOIN usergroup ON usergroup.id = ug1.usergroup_id
                                        JOIN "user" ON "user".id = ug1.user_id
                                        WHERE "user".email = $1
                                          AND usergroup.name = $2"#, &[&oldemail, &oldusergroup])?;

    if !singlegroup.is_empty() {
        println!();
        println!(
            "{} Deleting user '{}' from user group '{}' would leave this group empty.",
            "warning:".if_supports_color(Stdout, owo_colors::OwoColorize::yellow),
            &oldemail,
            &oldusergroup
        );
    };

    if !force {
        println!();
        print!(
            "Do you really want to remove user '{}' from user group '{}'? [y/N]: ",
            &oldemail, &oldusergroup
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    pgclient.query(query_string, &[&oldemail, &oldusergroup])?;

    info!(
        "({}) Deleted user '{}' from user group '{}'",
        &get_ssh_client(),
        &oldemail,
        &oldusergroup
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    if !singlegroup.is_empty() {
        println!();
        print!(
            "Do you want to remove the empty user group '{}' (and any associated user access)? [y/N]: ",
            &oldusergroup,
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!(
                "{} User group '{}' left empty.",
                "warning:".if_supports_color(Stdout, owo_colors::OwoColorize::yellow),
                &oldusergroup
            );
            std::process::exit(1);
        }

        if delete_usergroup(pgclient, Some(&oldusergroup), true).is_err() {
            exit_with_message("Could not delete user group.");
        };
    }

    Ok(())
}

pub fn delete_usergroup_from_usergroup(
    pgclient: &mut Client,
    subgroup: Option<&str>,
    supergroup: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Delete user group from user group");
    let query_string = r#"DELETE
                          FROM usergroup_usergroup
                          WHERE (subgroup_id,
                                 supergroup_id) =
                            (SELECT subgroup_id,
                                    supergroup_id
                             FROM usergroup_usergroup
                             JOIN usergroup AS ug1 ON ug1.id = subgroup_id
                             JOIN usergroup AS ug2 ON ug2.id = supergroup_id
                             WHERE ug1.name = $1
                               AND ug2.name = $2)"#;

    let oldsubgroupname = ask_for(
        &ListObject::UserGroup,
        subgroup,
        Some("(Member) group name ['?' for list]"),
        pgclient,
    );

    if oldsubgroupname.eq("") {
        exit_with_message("Group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM usergroup WHERE name = $1 LIMIT 1"#,
        &[&oldsubgroupname],
    )?;

    if res.is_empty() {
        exit_with_message("User group not found.");
    }

    let oldsupergroupname = ask_for(
        &ListObject::UserGroup,
        supergroup,
        Some("(Parent) group name ['?' for list]"),
        pgclient,
    );

    if oldsupergroupname.eq("") {
        exit_with_message("Group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM usergroup WHERE name = $1 LIMIT 1"#,
        &[&oldsupergroupname],
    )?;

    if res.is_empty() {
        exit_with_message("User group not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to remove user group '{}' from user group '{}'? [y/N]: ",
            &oldsubgroupname, &oldsupergroupname
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    pgclient.query(query_string, &[&oldsubgroupname, &oldsupergroupname])?;

    info!(
        "({}) Deleted user group '{}' from user group '{}'",
        &get_ssh_client(),
        &oldsubgroupname,
        &oldsupergroupname
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn delete_usergroup(
    pgclient: &mut Client,
    usergroup: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Delete user group");
    let query_string = r#"DELETE FROM usergroup WHERE name = $1"#;

    let oldusergroup = ask_for(&ListObject::UserGroup, usergroup, None, pgclient);

    if oldusergroup.eq("") {
        exit_with_message("User group cannot be empty.");
    }

    if pgclient
        .query(
            r#"SELECT name FROM usergroup WHERE name = $1 LIMIT 1"#,
            &[&oldusergroup],
        )?
        .is_empty()
    {
        exit_with_message("User group not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to delete user group '{}'? [y/N]: ",
            &oldusergroup
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    pgclient.query(query_string, &[&oldusergroup])?;

    info!(
        "({}) Deleted user group '{}'",
        &get_ssh_client(),
        &oldusergroup
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn delete_server_from_servergroup(
    pgclient: &mut Client,
    servername: Option<&str>,
    servergroup: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Delete server from server group");
    let query_string = r#"DELETE
                          FROM server_servergroup
                          WHERE server_servergroup =
                            (SELECT server_servergroup
                             FROM server_servergroup
                             JOIN server ON server_servergroup.server_id = server.id
                             JOIN servergroup ON server_servergroup.servergroup_id = servergroup.id
                             WHERE server.name = $1
                               AND servergroup.name = $2)"#;

    let oldservername = ask_for(&ListObject::ServerName, servername, None, pgclient);

    if oldservername.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    if pgclient
        .query(
            r#"SELECT name FROM server WHERE name = $1 LIMIT 1"#,
            &[&oldservername],
        )?
        .is_empty()
    {
        exit_with_message("Server not found.");
    }

    let oldservergroup = ask_for(&ListObject::ServerGroup, servergroup, None, pgclient);

    if oldservergroup.eq("") {
        exit_with_message("Group name cannot be empty.");
    }

    if pgclient
        .query(
            "SELECT name FROM servergroup WHERE name = $1 LIMIT 1",
            &[&oldservergroup],
        )?
        .is_empty()
    {
        exit_with_message("User group not found.");
    }

    if oldservergroup.eq("all") {
        println!();
        println!("{} Removing server '{}' from the built-in 'all' server group will disable default key management for this host.", "warning:".if_supports_color(Stdout, owo_colors::OwoColorize::yellow), &oldservername);
        println!("         Consider adding a dedicated server access for this host w/ user or user group access.");
    }

    let singlegroup = pgclient.query(r#"SELECT servergroup.name
                                        FROM server_servergroup AS ug1
                                        JOIN
                                          (SELECT servergroup_id
                                           FROM server_servergroup
                                           GROUP BY servergroup_id
                                           HAVING COUNT(servergroup_id) = 1) AS ug2 ON ug1.servergroup_id = ug2.servergroup_id
                                        JOIN servergroup ON servergroup.id = ug1.servergroup_id
                                        JOIN server ON server.id = ug1.server_id
                                        WHERE server.name = $1
                                          AND servergroup.name = $2"#, &[&oldservername, &oldservergroup])?;

    if !singlegroup.is_empty() && !oldservergroup.eq("all") {
        println!();
        println!(
            "{} Deleting server '{}' from server group '{}' will leave this group empty.",
            "warning:".if_supports_color(Stdout, owo_colors::OwoColorize::yellow),
            &oldservername,
            &oldservergroup
        );
    };

    if !force {
        println!();
        print!(
            "Do you really want to remove server '{}' from server group '{}'? [y/N]: ",
            &oldservername, &oldservergroup
        );
        let mut serverinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut serverinput).unwrap();
        if !serverinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    pgclient.query(query_string, &[&oldservername, &oldservergroup])?;

    info!(
        "({}) Deleted server '{}' from server group '{}'",
        &get_ssh_client(),
        &oldservername,
        &oldservergroup
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    if !singlegroup.is_empty() && !oldservergroup.eq("all") {
        println!();
        print!(
            "Do you want to remove the empty server group '{}' (and any associated server access)? [y/N]: ",
            &oldservergroup,
        );
        let mut serverinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut serverinput).unwrap();
        if !serverinput.trim().to_lowercase().eq("y") {
            println!(
                "{} Server group '{}' left empty.",
                "warning:".if_supports_color(Stdout, owo_colors::OwoColorize::yellow),
                &oldservergroup
            );
            std::process::exit(1);
        }

        if delete_servergroup(pgclient, Some(&oldservergroup), true).is_err() {
            exit_with_message("Could not delete server group.");
        };
    }

    Ok(())
}

pub fn delete_servergroup_from_servergroup(
    pgclient: &mut Client,
    subgroup: Option<&str>,
    supergroup: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Delete server group from server group");
    let query_string = r#"DELETE
                          FROM servergroup_servergroup
                          WHERE (subgroup_id,
                                 supergroup_id) =
                            (SELECT subgroup_id,
                                    supergroup_id
                             FROM servergroup_servergroup
                             JOIN servergroup AS sg1 ON sg1.id = subgroup_id
                             JOIN servergroup AS sg2 ON sg2.id = supergroup_id
                             WHERE sg1.name = $1
                               AND sg2.name = $2)"#;

    let oldsubgroupname = ask_for(
        &ListObject::ServerGroup,
        subgroup,
        Some("(Member) group name ['?' for list]"),
        pgclient,
    );

    if oldsubgroupname.eq("") {
        exit_with_message("Group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM servergroup WHERE name = $1 LIMIT 1"#,
        &[&oldsubgroupname],
    )?;

    if res.is_empty() {
        exit_with_message("Server group not found.");
    }

    let oldsupergroupname = ask_for(
        &ListObject::ServerGroup,
        supergroup,
        Some("(Parent) group name ['?' for list]"),
        pgclient,
    );

    if oldsupergroupname.eq("") {
        exit_with_message("Group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM servergroup WHERE name = $1 LIMIT 1"#,
        &[&oldsupergroupname],
    )?;

    if res.is_empty() {
        exit_with_message("Server group not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to remove server group '{}' from server group '{}'? [y/N]: ",
            &oldsubgroupname, &oldsupergroupname
        );
        let mut serverinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut serverinput).unwrap();
        if !serverinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    pgclient.query(query_string, &[&oldsubgroupname, &oldsupergroupname])?;

    info!(
        "({}) Deleted server group '{}' from server group '{}'",
        &get_ssh_client(),
        &oldsubgroupname,
        &oldsupergroupname
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn delete_servergroup(
    pgclient: &mut Client,
    servergroup: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Delete server group");
    let query_string = r#"DELETE FROM servergroup WHERE name = $1"#;

    let oldservergroup = ask_for(&ListObject::ServerGroup, servergroup, None, pgclient);

    if oldservergroup.eq("") {
        exit_with_message("Server group cannot be empty.");
    }

    if pgclient
        .query(
            r#"SELECT name FROM servergroup WHERE name = $1 LIMIT 1"#,
            &[&oldservergroup],
        )?
        .is_empty()
    {
        exit_with_message("Server group not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to delete server group '{}'? [y/N]: ",
            &oldservergroup
        );
        let mut serverinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut serverinput).unwrap();
        if !serverinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    pgclient.query(query_string, &[&oldservergroup])?;

    info!(
        "({}) Deleted server group '{}'",
        &get_ssh_client(),
        &oldservergroup
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn delete_serveraccess(
    pgclient: &mut Client,
    serveraccess: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Delete server access");
    let query_string = r#"DELETE FROM serveraccess WHERE name = $1"#;

    let oldserveraccess = ask_for(&ListObject::ServerAccess, serveraccess, None, pgclient);

    if oldserveraccess.eq("") {
        exit_with_message("Server access cannot be empty.");
    }

    if pgclient
        .query(
            r#"SELECT name FROM serveraccess WHERE name = $1"#,
            &[&oldserveraccess],
        )?
        .is_empty()
    {
        exit_with_message("Server access not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to delete server access '{}'? [y/N]: ",
            &oldserveraccess
        );
        let mut serverinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut serverinput).unwrap();
        if !serverinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    pgclient.query(query_string, &[&oldserveraccess])?;

    info!(
        "({}) Deleted server access '{}'",
        &get_ssh_client(),
        &oldserveraccess
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn delete_useraccess(
    pgclient: &mut Client,
    email: Option<&str>,
    usergroup: Option<&str>,
    serveraccess: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Delete user access");
    let query_string;

    let oldemail = if usergroup.is_none() {
        ask_for(&ListObject::UserEmail, email, None, pgclient)
    } else {
        String::new()
    };

    let oldemail = match &oldemail.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(oldemail),
    };

    let oldemail = if oldemail.is_none() {
        let oldgroupname = ask_for(&ListObject::UserGroup, usergroup, None, pgclient);

        match &oldgroupname.trim().to_lowercase()[..] {
            "" | "null" => {
                exit_with_message("User email and user group name cannot *both* be empty.")
            }
            _ => {
                if pgclient
                    .query(
                        r#"SELECT id FROM usergroup WHERE name = $1 LIMIT 1"#,
                        &[&oldgroupname],
                    )?
                    .is_empty()
                {
                    exit_with_message("User group not found.");
                }

                query_string = r#"DELETE
                                  FROM useraccess
                                  WHERE useraccess IN
                                    (SELECT useraccess
                                     FROM useraccess
                                     LEFT JOIN "user" ON "user".id = useraccess.user_id
                                     LEFT JOIN usergroup ON usergroup.id = useraccess.usergroup_id
                                     LEFT JOIN serveraccess ON serveraccess.id = useraccess.serveraccess_id
                                     WHERE serveraccess.name = $1
                                       AND "user".email IS NULL
                                       AND usergroup.name = $2)"#.to_string();
                Some(oldgroupname)
            }
        }
    } else {
        if pgclient
            .query(
                r#"SELECT id FROM "user" WHERE email = $1 LIMIT 1"#,
                &[&oldemail],
            )?
            .is_empty()
        {
            exit_with_message("User not found.");
        }

        query_string = r#"DELETE
                          FROM useraccess
                          WHERE useraccess IN
                            (SELECT useraccess
                             FROM useraccess
                             LEFT JOIN "user" ON "user".id = useraccess.user_id
                             LEFT JOIN usergroup ON usergroup.id = useraccess.usergroup_id
                             LEFT JOIN serveraccess ON serveraccess.id = useraccess.serveraccess_id
                             WHERE serveraccess.name = $1
                               AND "user".email = $2
                               AND usergroup.name IS NULL)"#
            .to_string();
        oldemail
    };

    let oldname = ask_for(&ListObject::ServerAccess, serveraccess, None, pgclient);

    if oldname.eq("") {
        exit_with_message("Server access name cannot be empty.");
    }

    if pgclient
        .query(
            r#"SELECT id FROM serveraccess WHERE name = $1 LIMIT 1"#,
            &[&oldname],
        )?
        .is_empty()
    {
        exit_with_message("Server access not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to delete user access '{}'? [y/N]: ",
            &oldname
        );
        let mut serverinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut serverinput).unwrap();
        if !serverinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    pgclient.query(&query_string, &[&oldname, &oldemail])?;

    info!(
        "({}) Deleted user access '{}' for '{}'",
        &get_ssh_client(),
        &oldname,
        &oldemail.unwrap_or_else(|| "-".to_string())
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}
