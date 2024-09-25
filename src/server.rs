use log::info;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, Error};
use prettytable::{format, Table};
use serde_derive::Serialize;
use std::io::{self, Write};
use std::net::IpAddr;

use crate::logging::get_ssh_client;
use crate::{ask_for, exit_with_message, server, set_or_ask_for, ListObject};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize)]
pub struct ServerQuery {
    pub id: i64,
    pub name: String,
    pub ip: IpAddr,
    pub disabled: String,
    pub use_dns: String,
    pub comment: Option<String>,
}

pub fn add(
    pgclient: &mut Client,
    servername: Option<&str>,
    ip: Option<&str>,
    disabled: bool,
    use_dns: bool,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Add server");
    let query_string = r"INSERT INTO server (name, ip, disabled, use_dns, comment)
                          VALUES ($1, $2, $3, $4, $5)";

    let newservername = ask_for(&ListObject::ServerName, servername, None, pgclient);

    if newservername.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    let res = pgclient.query(
        r"SELECT id FROM server WHERE name = $1 LIMIT 1",
        &[&newservername],
    )?;

    if !res.is_empty() {
        exit_with_message("Name already in use.");
    }

    let newip = set_or_ask_for(ip, "IP");

    let Ok(newip) = newip.parse::<IpAddr>() else {
        exit_with_message("This is not a valid IP address'")
    };

    let res = pgclient.query(r"SELECT id FROM server WHERE ip = $1 LIMIT 1", &[&newip])?;

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

pub fn add_to_servergroup(
    pgclient: &mut Client,
    servername: Option<&str>,
    servergroup: Option<&str>,
) -> Result<(), Error> {
    println!("Add server to server group");
    let query_string = r"INSERT INTO server_servergroup (server_id, servergroup_id)
                          SELECT server.id, servergroup.id
                          FROM server
                          JOIN servergroup
                          ON servergroup.name = $1
                          WHERE server.name = $2";

    let newgroupname = ask_for(&ListObject::ServerGroup, servergroup, None, pgclient);

    if newgroupname.eq("") {
        exit_with_message("Name cannot be empty.");
    }

    let res = pgclient.query(
        r"SELECT id FROM servergroup WHERE name = $1 LIMIT 1",
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
        r"SELECT id FROM server WHERE name = $1 LIMIT 1",
        &[&newservername],
    )?;

    if res.is_empty() {
        exit_with_message("Server not found.");
    }

    let res = pgclient.query(
        r"SELECT server_id FROM server_servergroup
           JOIN server ON server.id = server_servergroup.server_id
           JOIN servergroup ON servergroup.id = server_servergroup.servergroup_id
            WHERE server.name = $1
              AND servergroup.name = $2",
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
            server::add_to_servergroup(pgclient, None, Some(&newgroupname))?;
        }
    }

    Ok(())
}

pub fn delete(pgclient: &mut Client, servername: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Delete server");
    let query_string = r"DELETE FROM server WHERE name = $1";

    let oldservername = ask_for(&ListObject::ServerName, servername, None, pgclient);

    if oldservername.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    if pgclient
        .query(
            r"SELECT name FROM server WHERE name = $1 LIMIT 1",
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

#[allow(clippy::too_many_lines)]
pub fn delete_from_servergroup(
    pgclient: &mut Client,
    servername: Option<&str>,
    servergroup: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Delete server from server group");
    let query_string = r"DELETE
                          FROM server_servergroup
                          WHERE server_servergroup =
                            (SELECT server_servergroup
                             FROM server_servergroup
                             JOIN server ON server_servergroup.server_id = server.id
                             JOIN servergroup ON server_servergroup.servergroup_id = servergroup.id
                             WHERE server.name = $1
                               AND servergroup.name = $2)";

    let oldservername = ask_for(&ListObject::ServerName, servername, None, pgclient);

    if oldservername.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    if pgclient
        .query(
            r"SELECT name FROM server WHERE name = $1 LIMIT 1",
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

    let singlegroup = pgclient.query(r"SELECT servergroup.name
                                        FROM server_servergroup AS ug1
                                        JOIN
                                          (SELECT servergroup_id
                                           FROM server_servergroup
                                           GROUP BY servergroup_id
                                           HAVING COUNT(servergroup_id) = 1) AS ug2 ON ug1.servergroup_id = ug2.servergroup_id
                                        JOIN servergroup ON servergroup.id = ug1.servergroup_id
                                        JOIN server ON server.id = ug1.server_id
                                        WHERE server.name = $1
                                          AND servergroup.name = $2", &[&oldservername, &oldservergroup])?;

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

        if crate::servergroup::delete(pgclient, Some(&oldservergroup), true).is_err() {
            exit_with_message("Could not delete server group.");
        };
    }

    Ok(())
}

pub fn disable(pgclient: &mut Client, servername: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Disable server");
    let query_string = r"UPDATE server SET disabled = true WHERE name = $1";

    let oldname = ask_for(&ListObject::ServerName, servername, None, pgclient);

    if oldname.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    let res = pgclient.query(
        r"SELECT name FROM server WHERE name = $1 LIMIT 1",
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

    _ = pgclient.execute(query_string, &[&oldname])?;

    info!("({}) Disabled server '{}'", &get_ssh_client(), &oldname);

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn disable_dns(
    pgclient: &mut Client,
    servername: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Disable server DNS lookup");
    let query_string = r"UPDATE server SET use_dns = false WHERE name = $1";

    let oldname = ask_for(&ListObject::ServerName, servername, None, pgclient);

    if oldname.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    let res = pgclient.query(
        r"SELECT name FROM server WHERE name = $1 LIMIT 1",
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

    _ = pgclient.execute(query_string, &[&oldname])?;

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

pub fn enable(pgclient: &mut Client, servername: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Enable server");
    let query_string = r"UPDATE server SET disabled = false WHERE name = $1";

    let oldname = ask_for(&ListObject::ServerName, servername, None, pgclient);

    if oldname.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    let res = pgclient.query(
        r"SELECT name FROM server WHERE name = $1 LIMIT 1",
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

    _ = pgclient.execute(query_string, &[&oldname])?;

    info!("({}) Enabled server '{}'", &get_ssh_client(), &oldname);

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn enable_dns(
    pgclient: &mut Client,
    servername: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Enable server DNS lookup");
    let query_string = r"UPDATE server SET use_dns = true WHERE name = $1";

    let oldname = ask_for(&ListObject::ServerName, servername, None, pgclient);

    if oldname.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    let res = pgclient.query(
        r"SELECT name FROM server WHERE name = $1 LIMIT 1",
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

    _ = pgclient.execute(query_string, &[&oldname])?;

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

pub fn list(
    pgclient: &mut Client,
    servername: Option<&str>,
    ip: Option<&str>,
    id: Option<&str>,
    exact: bool,
    json: bool,
) -> std::result::Result<(), Error> {
    let query_string = r"SELECT id, name, ip, disabled::CHAR, use_dns::CHAR, comment
                          FROM server
                          ORDER BY id, name";

    let mut res = Vec::new();

    for row in pgclient.query(query_string, &[])? {
        res.push(ServerQuery {
            id: row.get("id"),
            name: row.get("name"),
            ip: row.get("ip"),
            disabled: row.get("disabled"),
            use_dns: row.get("use_dns"),
            comment: row.get("comment"),
        });
    }

    if let Some(name) = servername {
        if exact {
            res.retain(|x| x.name.eq(&name));
        } else {
            res.retain(|x| x.name.to_lowercase().contains(&name.to_lowercase()));
        }
    }

    if let Some(ip) = ip {
        if exact {
            res.retain(|x| x.ip.to_string().eq(&ip));
        } else {
            res.retain(|x| x.ip.to_string().contains(ip));
        }
    }

    if let Some(id) = id {
        let Ok(idint) = id.parse::<i64>() else {
            exit_with_message("Wrong ID format.")
        };
        res.retain(|x| x.id.eq(&idint));
    }

    if res.is_empty() {
        return Ok(());
    }

    if json {
        println!("{}", serde_json::to_string(&res).unwrap_or_default());
    } else {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        table.set_titles(row!["name", "ip", "disabled", "dns", "comment"]);

        for r in res {
            let (name, ip, disabled) = match &r.disabled[..] {
                "t" => (
                    format!(
                        "{}",
                        r.name
                            .if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
                    ),
                    format!(
                        "{}",
                        r.ip.if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
                    ),
                    format!(
                        "{}",
                        "yes".if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
                    ),
                ),
                _ => (r.name, r.ip.to_string(), "-".to_string()),
            };
            let use_dns = match &r.use_dns[..] {
                "t" => "yes".to_string(),
                _ => "-".to_string(),
            };
            table.add_row(row![
                name,
                ip,
                disabled,
                use_dns,
                r.comment.unwrap_or_else(|| "-".to_string())
            ]);
        }

        table.printstd();
    }

    Ok(())
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::similar_names)]
pub fn update(
    pgclient: &mut Client,
    servername: Option<&str>,
    newservername: Option<&str>,
    ip: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Update server");
    let query_string = r"UPDATE server
                          SET name = $1,
                              ip = $2,
                              comment = $3
                          WHERE name = $4";

    let oldservername = ask_for(&ListObject::ServerName, servername, None, pgclient);

    if oldservername.eq("") {
        exit_with_message("Server name cannot be empty.");
    }

    let res = pgclient.query(
        r"SELECT id, ip, comment FROM server WHERE name = $1 LIMIT 1",
        &[&oldservername],
    )?;

    if res.is_empty() {
        exit_with_message("Server not found.");
    }

    let oldserverid: i64 = res[0].get("id");
    let oldserverip: std::net::IpAddr = res[0].get("ip");
    let oldservercomment: Option<String> = res[0].get("comment");

    let mut newservername = ask_for(
        &ListObject::ServerName,
        newservername,
        Some(&format!(
            "New server name [<Enter>: '{}']",
            &oldservername.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        )),
        pgclient,
    );

    if newservername.to_lowercase().eq("") {
        newservername.clone_from(&oldservername);
    } else if !pgclient
        .query(
            r"SELECT id FROM server WHERE name = $1 AND id != $2",
            &[&newservername, &oldserverid],
        )?
        .is_empty()
    {
        exit_with_message("Server name already in use.");
    }

    let newip = set_or_ask_for(
        ip,
        &format!(
            "New IP [<Enter>: '{}']",
            &oldserverip.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    let newipaddr = if newip.eq("") {
        oldserverip
    } else {
        match newip.parse::<std::net::IpAddr>() {
            Ok(ip) => ip,
            Err(_) => exit_with_message("Not a valid IP address."),
        }
    };

    if !pgclient
        .query(
            r"SELECT ip FROM server WHERE ip = $1 AND id != $2",
            &[&newipaddr, &oldserverid],
        )?
        .is_empty()
    {
        exit_with_message("IP already in use.");
    }

    let newcomment = set_or_ask_for(
        comment,
        &format!(
            "New comment [<Enter>: '{}']",
            oldservercomment
                .as_ref()
                .map_or_else(|| "-".to_string(), std::string::ToString::to_string)
                .if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    let newcommentopt = if newcomment.to_lowercase().eq("") {
        oldservercomment
    } else if newcomment.trim().to_lowercase().eq("null") {
        None
    } else {
        Some(newcomment)
    };

    pgclient.query(
        query_string,
        &[&newservername, &newipaddr, &newcommentopt, &oldservername],
    )?;

    info!(
        "({}) Updated server '{}' ({}) -> '{}' ({})",
        &get_ssh_client(),
        &oldservername,
        &oldserverip,
        &newservername,
        &newipaddr
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, |t| t
            .if_supports_color(Stdout, owo_colors::OwoColorize::green))
    );

    Ok(())
}
