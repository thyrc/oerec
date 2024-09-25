use log::info;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, Error};
use prettytable::{format, Table};
use serde_derive::Serialize;
use std::io::{self, Write};
use std::net::IpAddr;

use crate::logging::get_ssh_client;
use crate::{ask_for, exit_with_message, set_or_ask_for, ListObject};

#[derive(Debug, Serialize)]
pub struct ServerAccessQuery {
    pub name: String,
    pub sshuser: String,
    pub server: Option<String>,
    pub ip: Option<IpAddr>,
    pub sshfrom: Option<String>,
    pub sshcommand: Option<String>,
    pub sshoption: Option<String>,
    pub servergroup: Option<String>,
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub fn add(
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
        r"SELECT id FROM serveraccess WHERE name = $1 LIMIT 1",
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
                    r"SELECT id FROM servergroup WHERE name = $1 LIMIT 1",
                    &[&newgroupname],
                )?;

                if res.is_empty() {
                    exit_with_message("Server group not found.");
                }

                query_string = r"INSERT INTO serveraccess (name, sshuser, sshfrom, sshcommand, sshoption, comment, servergroup_id)
                                  SELECT $1, $2, $3, $4, $5, $6, id
                                  FROM servergroup
                                  WHERE servergroup.name = $7".to_string();
                Some(newgroupname)
            }
        }
    } else {
        let res = pgclient.query(
            r"SELECT id FROM server WHERE name = $1 LIMIT 1",
            &[&newservername],
        )?;

        if res.is_empty() {
            exit_with_message("Server not found.");
        }

        query_string = r"INSERT INTO serveraccess (name, sshuser, sshfrom, sshcommand, sshoption, comment, server_id)
                          SELECT $1, $2, $3, $4, $5, $6, id
                          FROM server
                          WHERE server.name = $7".to_string();
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

pub fn delete(pgclient: &mut Client, serveraccess: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Delete server access");
    let query_string = r"DELETE FROM serveraccess WHERE name = $1";

    let oldserveraccess = ask_for(&ListObject::ServerAccess, serveraccess, None, pgclient);

    if oldserveraccess.eq("") {
        exit_with_message("Server access cannot be empty.");
    }

    if pgclient
        .query(
            r"SELECT name FROM serveraccess WHERE name = $1",
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

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub fn list(
    pgclient: &mut Client,
    serveraccess: Option<&str>,
    servername: Option<&str>,
    ip: Option<&str>,
    user: Option<&str>,
    servergroup: Option<&str>,
    exact: bool,
    json: bool,
) -> Result<(), Error> {
    let query_string = if serveraccess.is_some()
        || servername.is_some()
        || ip.is_some()
        || user.is_some()
        || servergroup.is_some()
        || json
    {
        // show members of groups as well
        r"SELECT serveraccess.name,
                  serveraccess.sshuser,
                  server.name AS server,
                  ip,
                  serveraccess.sshfrom,
                  serveraccess.sshcommand,
                  serveraccess.sshoption,
                  sg.name AS servergroup
           FROM serveraccess
           LEFT JOIN (serveraccess AS sa
                      JOIN (servergroup AS sg
                            JOIN server_servergroup AS stsg ON sg.id = stsg.servergroup_id) ON sa.servergroup_id = sg.id
                      OR sa.servergroup_id IN
                        (WITH RECURSIVE subgroups AS
                           (SELECT supergroup_id
                            FROM servergroup_servergroup
                            WHERE subgroup_id = sg.id
                            UNION SELECT s.supergroup_id
                            FROM servergroup_servergroup s
                            JOIN subgroups x ON x.supergroup_id = s.subgroup_id) SELECT DISTINCT supergroup_id
                         FROM subgroups)) ON serveraccess.id = sa.id
           JOIN server ON stsg.server_id = server.id
           OR serveraccess.server_id = server.id
           ORDER BY serveraccess.name,
                    serveraccess.sshuser,
                    server.name"
    } else {
        // only show individual names *or* group names (w/o listing every member)
        r"SELECT sa.name,
                  sa.sshuser,
                  s.name AS server,
                  s.ip,
                  sa.sshfrom,
                  sa.sshcommand,
                  sa.sshoption,
                  '-' AS servergroup
           FROM serveraccess AS sa,
                server AS s
           WHERE sa.server_id = s.id
           UNION
           SELECT sa.name,
                  sa.sshuser,
                  '-' AS server,
                  NULL::INET AS ip,
                  sa.sshfrom,
                  sa.sshcommand,
                  sa.sshoption,
                  sg.name AS servergroup
           FROM serveraccess AS sa,
                servergroup AS sg
           WHERE sa.servergroup_id = sg.id
           ORDER BY name,
                    server,
                    servergroup,
                    sshuser"
    };

    let mut res = Vec::new();

    for row in pgclient.query(query_string, &[])? {
        res.push(ServerAccessQuery {
            name: row.get("name"),
            sshuser: row.get("sshuser"),
            server: row.get("server"),
            ip: row.get("ip"),
            sshfrom: row.get("sshfrom"),
            sshcommand: row.get("sshcommand"),
            sshoption: row.get("sshoption"),
            servergroup: row.get("servergroup"),
        });
    }

    if let Some(name) = serveraccess {
        if exact {
            res.retain(|x| x.name.eq(&name));
        } else {
            res.retain(|x| x.name.to_lowercase().contains(&name.to_lowercase()));
        }
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

    if let Some(server) = servername {
        res.retain(|x| match &x.server {
            Some(x) => {
                if exact {
                    x.eq(&server)
                } else {
                    x.to_lowercase().contains(&server.to_lowercase())
                }
            }
            _ => false,
        });
    }

    if let Some(user) = user {
        if exact {
            res.retain(|x| x.sshuser.eq(&user));
        } else {
            res.retain(|x| x.sshuser.to_lowercase().contains(&user.to_lowercase()));
        }
    }

    if let Some(servergroup) = servergroup {
        res.retain(|x| match &x.servergroup {
            Some(x) => {
                if exact {
                    x.eq(&servergroup)
                } else {
                    x.to_lowercase().contains(&servergroup.to_lowercase())
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
            "name",
            "ssh user",
            "server",
            "ip",
            "ssh from",
            "ssh command",
            "ssh option",
            "server group"
        ]);

        for r in res {
            let ip = r.ip.map_or_else(|| "-".to_string(), |ip| ip.to_string());
            table.add_row(row![
                r.name,
                r.sshuser,
                r.server.unwrap_or_else(|| "-".to_string()),
                ip,
                r.sshfrom.unwrap_or_else(|| "-".to_string()),
                r.sshcommand.unwrap_or_else(|| "-".to_string()),
                r.sshoption.unwrap_or_else(|| "-".to_string()),
                r.servergroup.unwrap_or_else(|| "-".to_string())
            ]);
        }

        table.printstd();
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub fn update(
    pgclient: &mut Client,
    serveraccess: Option<&str>,
    newname: Option<&str>,
    sshuser: Option<&str>,
    sshfrom: Option<&str>,
    sshcommand: Option<&str>,
    sshoption: Option<&str>,
    servername: Option<&str>,
    servergroup: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Update server access");
    let query_string = r"UPDATE serveraccess
                          SET name = $1,
                              sshuser = $2,
                              sshfrom = $3,
                              sshcommand = $4,
                              sshoption = $5,
                              comment = $6,
                              server_id = $7,
                              servergroup_id = $8
                          WHERE id = $9";

    let newserveraccess = ask_for(&ListObject::ServerAccess, serveraccess, None, pgclient);

    if newserveraccess.eq("") {
        exit_with_message("Server access name cannot be empty.");
    }

    let res = pgclient.query(
        r"SELECT id,
                  name,
                  sshuser,
                  sshfrom,
                  sshcommand,
                  sshoption,
                  server_id,
                  servergroup_id,
                  comment
           FROM serveraccess
           WHERE name = $1
           LIMIT 1",
        &[&newserveraccess],
    )?;

    if res.is_empty() {
        exit_with_message("Server access not found.");
    }

    let oldserveraccessid: i64 = res[0].get("id");
    let oldserveraccessname: String = res[0].get("name");
    let oldserveraccesssshuser: String = res[0].get("sshuser");
    let oldserveraccesssshfrom: Option<String> = res[0].get("sshfrom");
    let oldserveraccesssshcommand: Option<String> = res[0].get("sshcommand");
    let oldserveraccesssshoption: Option<String> = res[0].get("sshoption");
    let oldserveraccessserverid: Option<i64> = res[0].get("server_id");
    let oldserveraccessservergroupid: Option<i64> = res[0].get("servergroup_id");
    let oldserveraccesscomment: Option<String> = res[0].get("comment");

    let mut newserveraccessname = ask_for(
        &ListObject::ServerAccess,
        newname,
        Some(&format!(
            "New server access name ['?' for list, <Enter>: '{}']",
            &oldserveraccessname.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        )),
        pgclient,
    );

    if newserveraccessname.eq("") {
        newserveraccessname.clone_from(&oldserveraccessname);
    }

    let mut newsshuser = set_or_ask_for(
        sshuser,
        &format!(
            "New SSH user [<Enter>: '{}'])",
            &oldserveraccesssshuser.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    if newsshuser.eq("") {
        newsshuser.clone_from(&oldserveraccesssshuser);
    }

    let newfrom = set_or_ask_for(
        sshfrom,
        &format!(
            "New sshfrom [<Enter>: '{}']",
            oldserveraccesssshfrom
                .as_ref()
                .map_or_else(|| "-".to_string(), std::string::ToString::to_string)
                .if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    let newfromopt = if newfrom.eq("") {
        oldserveraccesssshfrom.clone()
    } else if newfrom.trim().to_lowercase().eq("null") {
        None
    } else {
        Some(newfrom)
    };

    let newcommand = set_or_ask_for(
        sshcommand,
        &format!(
            "New sshcommand [<Enter>: '{}']",
            oldserveraccesssshcommand
                .as_ref()
                .map_or_else(|| "-".to_string(), std::string::ToString::to_string)
                .if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    let newcommandopt = if newcommand.eq("") {
        oldserveraccesssshcommand
    } else if newcommand.trim().to_lowercase().eq("null") {
        None
    } else {
        Some(newcommand)
    };

    let newoption = set_or_ask_for(
        sshoption,
        &format!(
            "New sshoption [<Enter>: '{}']",
            oldserveraccesssshoption
                .as_ref()
                .map_or_else(|| "-".to_string(), std::string::ToString::to_string)
                .if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    let newoptionopt = if newoption.eq("") {
        oldserveraccesssshoption
    } else if newoption.trim().to_lowercase().eq("null") {
        None
    } else {
        Some(newoption)
    };

    let oldserveraccessservername: String = if oldserveraccessserverid.is_some() {
        let res = pgclient.query(
            r"SELECT name FROM server WHERE id = $1 LIMIT 1",
            &[&oldserveraccessserverid],
        )?;
        res[0].get("name")
    } else {
        '-'.to_string()
    };

    let mut newservername = ask_for(
        &ListObject::ServerName,
        servername,
        Some(&format!(
            "New server name ['?' for list, 'null' to clear, <Enter>: '{}']",
            &oldserveraccessservername.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        )),
        pgclient,
    );

    let newserverid = match &newservername.trim().to_lowercase()[..] {
        "" => {
            newservername.clone_from(&oldserveraccessservername);
            oldserveraccessserverid
        }
        "null" => None,
        _ => {
            let res = pgclient.query(
                r"SELECT id FROM server WHERE name = $1 LIMIT 1",
                &[&newservername],
            )?;

            if res.is_empty() {
                exit_with_message("Server not found.");
            }

            Some(res[0].get("id"))
        }
    };

    let oldserveraccessservergroupname = if oldserveraccessservergroupid.is_some() {
        let res = pgclient.query(
            r"SELECT name FROM servergroup WHERE id = $1 LIMIT 1",
            &[&oldserveraccessservergroupid],
        )?;
        res[0].get("name")
    } else {
        '-'.to_string()
    };

    let mut newgroupname = String::new();
    let newservergroupid = if newserverid.is_none() {
        newgroupname = ask_for(
            &ListObject::ServerGroup,
            servergroup,
            Some(&format!(
                "New server group name ['?' for list, <Enter>: '{}']",
                &oldserveraccessservergroupname
                    .if_supports_color(Stdout, owo_colors::OwoColorize::green)
            )),
            pgclient,
        );

        match &newgroupname.trim().to_lowercase()[..] {
            "" => {
                if oldserveraccessservergroupid.is_none() {
                    exit_with_message("Server name and server group name cannot *both* be empty.")
                }
                newgroupname.clone_from(&oldserveraccessservergroupname);
                oldserveraccessservergroupid
            }
            "null" => {
                exit_with_message("Server name and server group name cannot *both* be empty.")
            }
            _ => {
                let res = pgclient.query(
                    r"SELECT id FROM servergroup WHERE name = $1 LIMIT 1",
                    &[&newgroupname],
                )?;

                if res.is_empty() {
                    exit_with_message("Server group not found.");
                }

                Some(res[0].get("id"))
            }
        }
    } else {
        None
    };

    let newcomment = set_or_ask_for(
        comment,
        &format!(
            "New comment [<Enter>: '{}']",
            oldserveraccesscomment
                .as_ref()
                .map_or_else(|| "-".to_string(), std::string::ToString::to_string)
                .if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    let newcommentopt = if newcomment.to_lowercase().eq("") {
        oldserveraccesscomment
    } else if newcomment.trim().to_lowercase().eq("null") {
        None
    } else {
        Some(newcomment)
    };

    pgclient.query(
        query_string,
        &[
            &newserveraccessname,
            &newsshuser,
            &newfromopt,
            &newcommandopt,
            &newoptionopt,
            &newcommentopt,
            &newserverid,
            &newservergroupid,
            &oldserveraccessid,
        ],
    )?;

    info!(
        "({}) Updated serveraccess '{}' (user: '{}', from: '{}', server: '{}', server group '{}') -> '{}' (user: '{}', from: '{}', server: '{}', server group: '{}')",
        &get_ssh_client(),
        &oldserveraccessname,
        &oldserveraccesssshuser,
        &oldserveraccesssshfrom.unwrap_or_else(|| "-".to_string()),
        &oldserveraccessservername,
        &oldserveraccessservergroupname,
        &newserveraccessname,
        &newsshuser,
        &newfromopt.unwrap_or_else(|| "-".to_string()),
        &newservername,
        match &newgroupname[..] {
            "" => "-",
            _ => &newgroupname
        }
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, |t| t
            .if_supports_color(Stdout, owo_colors::OwoColorize::green))
    );

    Ok(())
}
