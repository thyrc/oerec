use log::info;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, Error};
use prettytable::{format, Table};
use serde_derive::Serialize;
use std::io::{self, Write};
use std::net::IpAddr;

use crate::logging::get_ssh_client;
use crate::{ask_for, exit_with_message, server, servergroup, set_or_ask_for, ListObject};

#[derive(Debug, Serialize)]
struct ServerGroupQuery {
    pub servergroup: String,
    pub member: Option<String>,
    pub ip: Option<IpAddr>,
    pub comment: Option<String>,
    pub subgroups: Option<String>,
}

pub fn add(
    pgclient: &mut Client,
    servergroup: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Add server group");
    let query_string = r"INSERT INTO servergroup (name, comment) VALUES ($1, $2)";

    let newname = ask_for(&ListObject::ServerGroup, servergroup, None, pgclient);

    if newname.eq("") {
        exit_with_message("Server group name cannot be empty.");
    }

    let res = pgclient.query(
        r"SELECT id FROM servergroup WHERE name = $1 LIMIT 1",
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
            server::add_to_servergroup(pgclient, None, Some(&newname))?;
        }
    }

    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub fn add_to_servergroup(
    pgclient: &mut Client,
    subgroup: Option<&str>,
    supergroup: Option<&str>,
) -> Result<(), Error> {
    println!("Add server group to server group");
    let query_string = r"INSERT INTO servergroup_servergroup (subgroup_id, supergroup_id)
                          SELECT sg1.id,
                            (SELECT sg2.id
                             FROM servergroup sg2
                             WHERE sg2.name = $2)
                          FROM servergroup sg1
                          WHERE sg1.name = $1";

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
        r"SELECT id FROM servergroup WHERE name = $1 LIMIT 1",
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
        r"SELECT id FROM servergroup WHERE name = $1 LIMIT 1",
        &[&newsupergroupname],
    )?;

    if res.is_empty() {
        exit_with_message("Server group not found.");
    }

    let res = pgclient.query(
        r"SELECT subgroup_id, supergroup_id
           FROM servergroup_servergroup
           JOIN servergroup AS sg1 ON sg1.id = supergroup_id
           JOIN servergroup AS sg2 ON sg2.id = subgroup_id
           WHERE sg1.name = $1
             AND sg2.name = $2",
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
            servergroup::add_to_servergroup(pgclient, None, Some(&newsupergroupname))?;
        }
    }

    Ok(())
}

pub fn delete(pgclient: &mut Client, servergroup: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Delete server group");
    let query_string = r"DELETE FROM servergroup WHERE name = $1";

    let oldservergroup = ask_for(&ListObject::ServerGroup, servergroup, None, pgclient);

    if oldservergroup.eq("") {
        exit_with_message("Server group cannot be empty.");
    }

    if pgclient
        .query(
            r"SELECT name FROM servergroup WHERE name = $1 LIMIT 1",
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

#[allow(clippy::module_name_repetitions)]
pub fn delete_from_servergroup(
    pgclient: &mut Client,
    subgroup: Option<&str>,
    supergroup: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Delete server group from server group");
    let query_string = r"DELETE
                          FROM servergroup_servergroup
                          WHERE (subgroup_id,
                                 supergroup_id) =
                            (SELECT subgroup_id,
                                    supergroup_id
                             FROM servergroup_servergroup
                             JOIN servergroup AS sg1 ON sg1.id = subgroup_id
                             JOIN servergroup AS sg2 ON sg2.id = supergroup_id
                             WHERE sg1.name = $1
                               AND sg2.name = $2)";

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
        r"SELECT id FROM servergroup WHERE name = $1 LIMIT 1",
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
        r"SELECT id FROM servergroup WHERE name = $1 LIMIT 1",
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

#[allow(clippy::too_many_arguments)]
#[allow(clippy::fn_params_excessive_bools)]
#[allow(clippy::too_many_lines)]
pub fn list(
    pgclient: &mut Client,
    servergroup: Option<&str>,
    servername: Option<&str>,
    ip: Option<&str>,
    exact: bool,
    all: bool,
    empty: bool,
    json: bool,
) -> Result<(), Error> {
    let query_string = if servername.is_some()
        || ip.is_some()
        || servergroup.is_some()
        || empty
        || json
    {
        r"SELECT DISTINCT servergroup.name AS servergroup,
                           server.name AS member,
                           ip,
                           servergroup.comment,
                           CASE
                               WHEN (supergroup_id IS NOT NULL) THEN
                                      (WITH RECURSIVE subgroups(id, PATH) AS
                                         (SELECT subgroup_id,
                                            (SELECT name
                                             FROM servergroup
                                             WHERE servergroup.id = subgroup_id)::TEXT AS PATH
                                          FROM servergroup_servergroup
                                          WHERE supergroup_id = servergroup.id
                                          UNION SELECT s.subgroup_id,
                                                       CONCAT(
                                                                (SELECT name
                                                                 FROM servergroup
                                                                 WHERE servergroup.id = supergroup_id), ' <- ',
                                                                (SELECT name
                                                                 FROM servergroup
                                                                 WHERE servergroup.id = subgroup_id))
                                          FROM servergroup_servergroup s
                                          JOIN subgroups x ON x.id = s.supergroup_id) SELECT string_agg(PATH, ', ')
                                       FROM subgroups
                                       WHERE id = subgroup_id)
                           END AS subgroups
           FROM servergroup
           LEFT JOIN server_servergroup AS stsg
           JOIN server ON stsg.server_id = server.id ON servergroup.id = stsg.servergroup_id
           OR servergroup.id IN
             (WITH RECURSIVE subgroups AS
                (SELECT supergroup_id
                 FROM servergroup_servergroup
                 WHERE subgroup_id = stsg.servergroup_id
                 UNION SELECT s.supergroup_id
                 FROM servergroup_servergroup s
                 JOIN subgroups x ON x.supergroup_id = s.subgroup_id) SELECT DISTINCT supergroup_id
              FROM subgroups)
           LEFT JOIN servergroup_servergroup ON subgroup_id = servergroup_id
           ORDER BY servergroup,
                    server.name"
    } else {
        r"SELECT DISTINCT servergroup.name AS servergroup,
                           NULL AS member,
                           NULL::INET AS ip,
                           servergroup.comment,
                           CASE
                               WHEN (subgroup_id IS NOT NULL) THEN 'yes'
                           END AS subgroups
           FROM servergroup
           LEFT JOIN servergroup_servergroup ON id = supergroup_id
           ORDER BY servergroup"
    };

    let mut res = Vec::new();

    for row in pgclient.query(query_string, &[])? {
        res.push(ServerGroupQuery {
            servergroup: row.get("servergroup"),
            member: row.get("member"),
            ip: row.get("ip"),
            comment: row.get("comment"),
            subgroups: row.get("subgroups"),
        });
    }

    if !all {
        res.retain(|x| !x.servergroup.eq("all"));
    }

    if empty {
        res.retain(|x| x.ip.is_none());
    }

    if let Some(name) = servergroup {
        if exact {
            res.retain(|x| x.servergroup.eq(&name));
        } else {
            res.retain(|x| x.servergroup.to_lowercase().contains(&name.to_lowercase()));
        }
    }

    if let Some(servername) = servername {
        res.retain(|x| match &x.member {
            Some(x) => {
                if exact {
                    x.eq(&servername)
                } else {
                    x.to_lowercase().contains(&servername.to_lowercase())
                }
            }
            _ => false,
        });
    }

    if let Some(ip) = ip {
        res.retain(|x| match &x.ip {
            Some(x) => {
                if exact {
                    x.to_string().eq(&ip)
                } else {
                    x.to_string().contains(ip)
                }
            }
            _ => false,
        });
    }

    if res.is_empty() {
        return Ok(());
    }

    if json {
        println!("{}", serde_json::to_string(&res).unwrap_or_default());
    } else {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        table.set_titles(row![
            "servergroup",
            "member",
            "ip",
            "comment",
            "w/ subgroups"
        ]);

        for r in res {
            let ip = r.ip.map_or_else(|| "-".to_string(), |ip| ip.to_string());
            table.add_row(row![
                r.servergroup,
                r.member.unwrap_or_else(|| "-".to_string()),
                ip,
                r.comment.unwrap_or_else(|| "-".to_string()),
                r.subgroups.unwrap_or_else(|| "-".to_string()),
            ]);
        }

        table.printstd();
    }

    Ok(())
}

pub fn update(
    pgclient: &mut Client,
    servergroup: Option<&str>,
    newservergroup: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Update server group");
    let query_string = r"UPDATE servergroup
                          SET name = $1,
                              comment = $2
                          WHERE id = $3";

    let servergroupname = ask_for(&ListObject::ServerGroup, servergroup, None, pgclient);

    if servergroupname.eq("") {
        exit_with_message("Server group name cannot be empty.");
    }

    let res = pgclient.query(
        r"SELECT id, name, comment FROM servergroup WHERE name = $1 LIMIT 1",
        &[&servergroupname],
    )?;

    if res.is_empty() {
        exit_with_message("Server group not found.");
    }

    let oldservergroupid: i64 = res[0].get("id");
    let oldservergroupname: String = res[0].get("name");
    let oldservergroupcomment: Option<String> = res[0].get("comment");

    let mut newservergroupname = ask_for(
        &ListObject::ServerGroup,
        newservergroup,
        Some(&format!(
            "New server group name [<Enter>: '{}']",
            oldservergroupname.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        )),
        pgclient,
    );

    if newservergroupname.to_lowercase().eq("") {
        newservergroupname.clone_from(&oldservergroupname);
    } else if !pgclient
        .query(
            r"SELECT id FROM servergroup WHERE name = $1 AND id != $2",
            &[&newservergroupname, &oldservergroupid],
        )?
        .is_empty()
    {
        exit_with_message("Server group name already in use.");
    }

    let newcomment = set_or_ask_for(
        comment,
        &format!(
            "New comment [<Enter>: '{}']",
            oldservergroupcomment
                .as_ref()
                .map_or_else(|| "-".to_string(), std::string::ToString::to_string)
                .if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    let newcommentopt = if newcomment.to_lowercase().eq("") {
        oldservergroupcomment
    } else if newcomment.trim().to_lowercase().eq("null") {
        None
    } else {
        Some(newcomment)
    };

    pgclient.query(
        query_string,
        &[&newservergroupname, &newcommentopt, &oldservergroupid],
    )?;

    info!(
        "({}) Updated server group '{}' -> '{}'",
        &get_ssh_client(),
        &oldservergroupname,
        &newservergroupname
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, |t| t
            .if_supports_color(Stdout, owo_colors::OwoColorize::green))
    );

    Ok(())
}
