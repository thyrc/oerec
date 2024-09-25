use log::info;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, Error};
use prettytable::{format, Table};
use serde_derive::Serialize;
use std::io::{self, Write};
use std::net::IpAddr;
use time::{format_description, Date, PrimitiveDateTime, Time};

use crate::logging::get_ssh_client;
use crate::{ask_for, exit_with_message, set_or_ask_for, ListObject};

#[derive(Debug, Serialize)]
struct UserAccessQuery {
    pub email: String,
    pub sshuser: String,
    pub serveraccess: String,
    pub ip: Option<IpAddr>,
    pub servername: Option<String>,
    pub usergroup: Option<String>,
    pub servergroup: Option<String>,
    pub until: String,
}

#[allow(clippy::too_many_lines)]
pub fn add(
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
                    r"SELECT id FROM usergroup WHERE name = $1 LIMIT 1",
                    &[&newgroupname],
                )?;

                if res.is_empty() {
                    exit_with_message("User group not found.");
                }

                query_string =
                    r"INSERT INTO useraccess (usergroup_id, serveraccess_id, comment, best_before)
                                  SELECT usergroup.id, serveraccess.id, $1, $2
                                  FROM usergroup,serveraccess
                                  WHERE usergroup.name = $3
                                    AND serveraccess.name = $4"
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

    let res = pgclient.query(r"SELECT id FROM serveraccess WHERE name = $1", &[&newname])?;

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
                let Ok(date) = Date::parse(
                    newuntil.split(' ').collect::<Vec<&str>>()[0],
                    &format_description::parse("[year]-[month]-[day]")
                        .expect("BUG: DateTimeFormatDesc"),
                ) else {
                    exit_with_message("Could not parse date.")
                };
                PrimitiveDateTime::new(date, Time::MIDNIGHT)
            } else if newuntil.split(' ').count() == 2 {
                let Ok(date) = Date::parse(
                    newuntil.split(' ').collect::<Vec<&str>>()[0],
                    &format_description::parse("[year]-[month]-[day]")
                        .expect("BUG: DateTimeFormatDesc"),
                ) else {
                    exit_with_message("Could not parse date.")
                };
                let Ok(time) = Time::parse(
                    newuntil.split(' ').collect::<Vec<&str>>()[1],
                    &format_description::parse("[hour]:[minute]:[second]")
                        .expect("BUG: DateTimeFormatDesc"),
                ) else {
                    exit_with_message("Could not parse time.")
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

#[allow(clippy::too_many_lines)]
pub fn delete(
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
                        r"SELECT id FROM usergroup WHERE name = $1 LIMIT 1",
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
            r"SELECT id FROM serveraccess WHERE name = $1 LIMIT 1",
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

#[allow(clippy::too_many_arguments)]
#[allow(clippy::fn_params_excessive_bools)]
#[allow(clippy::too_many_lines)]
pub fn list(
    pgclient: &mut Client,
    email: Option<&str>,
    servername: Option<&str>,
    ip: Option<&str>,
    user: Option<&str>,
    serveraccess: Option<&str>,
    servergroup: Option<&str>,
    usergroup: Option<&str>,
    exact: bool,
    expired: bool,
    disabled: bool,
    json: bool,
) -> Result<(), Error> {
    let mut query_string = if email.is_some()
        || servername.is_some()
        || ip.is_some()
        || user.is_some()
        || serveraccess.is_some()
        || servergroup.is_some()
        || usergroup.is_some()
        || disabled
        || json
    {
        // show members of groups as well
        r#"SELECT "user".email,
                  serveraccess.sshuser,
                  serveraccess.name AS serveraccess,
                  ip,
                  server.name AS servername,
                  ug.name AS usergroup,
                  sg.name AS servergroup,
                  useraccess.best_before::VARCHAR AS UNTIL
           FROM useraccess
           LEFT JOIN (useraccess AS ua
                      JOIN (usergroup AS ug
                            JOIN user_usergroup AS utug ON ug.id = utug.usergroup_id) ON ua.usergroup_id = ug.id
                      OR ua.usergroup_id IN
                        (WITH RECURSIVE subgroups AS
                           (SELECT supergroup_id
                            FROM usergroup_usergroup
                            WHERE subgroup_id = ug.id
                            UNION SELECT u.supergroup_id
                            FROM usergroup_usergroup u
                            JOIN subgroups x ON x.supergroup_id = u.subgroup_id) SELECT DISTINCT supergroup_id
                         FROM subgroups)) ON useraccess.id = ua.id
           JOIN "user" ON utug.user_id = "user".id
           OR useraccess.user_id = "user".id
           JOIN serveraccess
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
                         FROM subgroups)) ON serveraccess.id = sa.id ON useraccess.serveraccess_id = serveraccess.id
           JOIN server ON stsg.server_id = server.id
           OR serveraccess.server_id = server.id
           WHERE NOT "user".disabled
             AND NOT server.disabled
             AND useraccess.best_before >= NOW()
           ORDER BY "user".email,
                    serveraccess.sshuser,
                    server.name"#.to_string()
    } else {
        // only show individual users *or* usergroup names (w/o listing every member)
        r#"SELECT u.email,
                  '-' AS usergroup,
                  sa.name AS serveraccess,
                  sa.sshuser,
                  s.ip,
                  s.name AS servername,
                  '-' AS servergroup,
                  ua.best_before::VARCHAR AS UNTIL
           FROM useraccess AS ua,
                serveraccess AS sa,
                "user" AS u,
                server AS s
           WHERE ua.serveraccess_id = sa.id
             AND ua.user_id = u.id
             AND sa.server_id = s.id
             AND NOT s.disabled
             AND NOT u.disabled
             AND ua.best_before >= NOW()
           UNION
           SELECT u.email,
                  '-' AS usergroup,
                  sa.name AS serveraccess,
                  sa.sshuser,
                  NULL::INET AS ip,
                  '-' AS servername,
                  sg.name AS servergroup,
                  ua.best_before::VARCHAR AS UNTIL
           FROM useraccess AS ua,
                serveraccess AS sa,
                "user" AS u,
                servergroup AS sg
           WHERE ua.serveraccess_id = sa.id
             AND ua.user_id = u.id
             AND sa.servergroup_id = sg.id
             AND NOT u.disabled
             AND ua.best_before >= NOW()
           UNION
           SELECT '-' AS email,
                  ug.name AS usergroup,
                  sa.name AS serveraccess,
                  sa.sshuser,
                  s.ip,
                  s.name AS servername,
                  '-' AS servergroup,
                  ua.best_before::VARCHAR AS UNTIL
           FROM useraccess AS ua,
                serveraccess AS sa,
                usergroup AS ug,
                server AS s
           WHERE ua.serveraccess_id = sa.id
             AND ua.usergroup_id = ug.id
             AND sa.server_id = s.id
             AND NOT s.disabled
             AND ua.best_before >= NOW()
           UNION
           SELECT '-' AS email,
                  ug.name AS usergroup,
                  sa.name AS serveraccess,
                  sa.sshuser,
                  NULL::INET AS ip,
                  '-' AS servername,
                  sg.name AS servergroup,
                  ua.best_before::VARCHAR AS UNTIL
           FROM useraccess AS ua,
                serveraccess AS sa,
                usergroup AS ug,
                servergroup AS sg
           WHERE ua.serveraccess_id = sa.id
             AND ua.usergroup_id = ug.id
             AND sa.servergroup_id = sg.id
             AND ua.best_before >= NOW()
           ORDER BY email DESC"#
            .to_string()
    };

    if expired {
        query_string = query_string.replace(">=", "<");
    }

    if disabled {
        query_string = query_string.replace(r#"NOT "user".disabled"#, r#"("user".disabled"#);
        query_string = query_string.replace(r"AND NOT server.disabled", r"OR server.disabled)");
    }

    let mut res = Vec::new();

    for row in pgclient.query(&query_string, &[])? {
        res.push(UserAccessQuery {
            email: row.get("email"),
            sshuser: row.get("sshuser"),
            serveraccess: row.get("serveraccess"),
            ip: row.get("ip"),
            servername: row.get("servername"),
            usergroup: row.get("usergroup"),
            servergroup: row.get("servergroup"),
            until: row.get("until"),
        });
    }

    if let Some(email) = email {
        if exact {
            res.retain(|x| x.email.eq(&email));
        } else {
            res.retain(|x| x.email.to_lowercase().contains(&email.to_lowercase()));
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

    if let Some(servername) = servername {
        res.retain(|x| match &x.servername {
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

    if let Some(user) = user {
        if exact {
            res.retain(|x| x.sshuser.eq(&user));
        } else {
            res.retain(|x| x.sshuser.to_lowercase().contains(&user.to_lowercase()));
        }
    }

    if let Some(serveraccess) = serveraccess {
        if exact {
            res.retain(|x| x.serveraccess.eq(&serveraccess));
        } else {
            res.retain(|x| {
                x.serveraccess
                    .to_lowercase()
                    .contains(&serveraccess.to_lowercase())
            });
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

    if let Some(usergroup) = usergroup {
        res.retain(|x| match &x.usergroup {
            Some(x) => {
                if exact {
                    x.eq(&usergroup)
                } else {
                    x.to_lowercase().contains(&usergroup.to_lowercase())
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
            "email",
            "user group",
            "serveraccess",
            "ssh user",
            "ip",
            "servername",
            "server group",
            "until"
        ]);

        for r in res {
            let (usergroup, email) = match r.usergroup {
                Some(usergroup) if usergroup.ne("-") => (
                    usergroup
                        .if_supports_color(Stdout, owo_colors::OwoColorize::blue)
                        .to_string(),
                    r.email,
                ),
                _ => (
                    "-".to_string(),
                    r.email
                        .if_supports_color(Stdout, owo_colors::OwoColorize::blue)
                        .to_string(),
                ),
            };
            let ip = r.ip.map_or_else(|| "-".to_string(), |ip| ip.to_string());
            table.add_row(row![
                email,
                usergroup,
                r.serveraccess
                    .if_supports_color(Stdout, owo_colors::OwoColorize::blue),
                r.sshuser,
                ip,
                r.servername.unwrap_or_else(|| "-".to_string()),
                r.servergroup.unwrap_or_else(|| "-".to_string()),
                r.until
            ]);
        }

        table.printstd();
    }

    Ok(())
}
