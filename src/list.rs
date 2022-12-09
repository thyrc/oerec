#![allow(clippy::module_name_repetitions)]
use crate::exit_with_message;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, Error};
use prettytable::{format, Table};

use crate::generate::generate_serverauth;

use crate::schema::{
    ServerAccessQuery, ServerGroupQuery, ServerQuery, SshKeysQuery, UserAccessQuery,
    UserGroupQuery, UserQuery,
};

pub fn list_user(
    pgclient: &mut Client,
    email: Option<&str>,
    name: Option<&str>,
    id: Option<&str>,
    exact: bool,
    json: bool,
) -> std::result::Result<(), Error> {
    let query_string = r#"SELECT id, email, name, type::VARCHAR, disabled::CHAR, comment
                          FROM "user"
                          ORDER BY id"#;

    let mut res = Vec::new();

    for row in pgclient.query(query_string, &[])? {
        res.push(UserQuery {
            id: row.get("id"),
            email: row.get("email"),
            name: row.get("name"),
            usertype: row.get("type"),
            disabled: row.get("disabled"),
            comment: row.get("comment"),
        });
    }

    if let Some(email) = email {
        if exact {
            res.retain(|x| x.email.eq(&email));
        } else {
            res.retain(|x| x.email.to_lowercase().contains(&email.to_lowercase()));
        }
    }

    if let Some(name) = name {
        if exact {
            res.retain(|x| x.name.eq(&name));
        } else {
            res.retain(|x| x.name.to_lowercase().contains(&name.to_lowercase()));
        }
    }

    if let Some(id) = id {
        let idint = match id.parse::<i64>() {
            Ok(i) => i,
            Err(_) => exit_with_message("Wrong ID format."),
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

        table.set_titles(row!["id", "email", "name", "type", "disabled", "comment"]);

        for r in res {
            let (email, name, disabled) = match &r.disabled[..] {
                "t" => (
                    format!(
                        "{}",
                        r.email
                            .if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
                    ),
                    format!(
                        "{}",
                        r.name
                            .if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
                    ),
                    format!(
                        "{}",
                        "yes".if_supports_color(Stdout, owo_colors::OwoColorize::yellow)
                    ),
                ),
                _ => (r.email, r.name, "-".to_string()),
            };
            table.add_row(row![
                r.id,
                email,
                name,
                r.usertype,
                disabled,
                r.comment.unwrap_or_else(|| "-".to_string())
            ]);
        }

        table.printstd();
    }

    Ok(())
}

pub fn list_key(
    pgclient: &mut Client,
    email: Option<&str>,
    fingerprint: Option<&str>,
    with_key: bool,
    id: Option<&str>,
    json: bool,
) -> std::result::Result<(), Error> {
    let query_string = r#"SELECT sshkeys.id, "user".email, sshkey, sshkeys.fingerprint, sshkeys.comment
                          FROM sshkeys, "user"
                          WHERE sshkeys.user_id = "user".id
                          ORDER BY sshkeys.id"#;

    let mut res = Vec::new();

    for row in pgclient.query(query_string, &[])? {
        res.push(SshKeysQuery {
            id: row.get("id"),
            email: row.get("email"),
            sshkey: row.get("sshkey"),
            fingerprint: row.get("fingerprint"),
            comment: row.get("comment"),
        });
    }

    if let Some(email) = email {
        res.retain(|x| x.email.to_lowercase().contains(&email.to_lowercase()));
    }

    if let Some(fingerprint) = fingerprint {
        res.retain(|x| x.fingerprint.contains(fingerprint));
    }

    if let Some(id) = id {
        let idint = match id.parse::<i64>() {
            Ok(i) => i,
            Err(_) => exit_with_message("Wrong ID format."),
        };
        res.retain(|x| x.id.eq(&idint));
    }

    if res.is_empty() {
        return Ok(());
    }

    if json {
        println!("{}", serde_json::to_string(&res).unwrap_or_default());
    } else if with_key {
        for r in res {
            let comment = r.comment.map_or_else(|| "-".to_string(), |comment| comment);
            println!("id:          {}", r.id);
            println!("email:       {}", r.email);
            println!("ssh key:     {}", r.sshkey);
            println!("fingerprint: {}", r.fingerprint);
            println!("comment:     {}", comment);
            println!("---");
        }
    } else {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        table.set_titles(row!["id", "email", "fingerprint", "comment"]);

        for r in res {
            table.add_row(row![
                r.id,
                r.email,
                r.fingerprint,
                r.comment.unwrap_or_else(|| "-".to_string())
            ]);
        }

        table.printstd();
    }

    Ok(())
}

pub fn list_server(
    pgclient: &mut Client,
    name: Option<&str>,
    ip: Option<&str>,
    id: Option<&str>,
    exact: bool,
    json: bool,
) -> std::result::Result<(), Error> {
    let query_string = r#"SELECT id, name, ip, disabled::CHAR, use_dns::CHAR, comment
                          FROM server
                          ORDER BY id, name"#;

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

    if let Some(name) = name {
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
        let idint = match id.parse::<i64>() {
            Ok(i) => i,
            Err(_) => exit_with_message("Wrong ID format."),
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

        table.set_titles(row!["id", "name", "ip", "disabled", "dns", "comment"]);

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
                r.id,
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

pub fn list_serverauth(
    pgclient: &mut Client,
    ip: Option<&str>,
    servername: Option<&str>,
) -> Result<(), Error> {
    let query_string = r#"SELECT ip FROM server WHERE name = $1"#;

    let serverip: Option<&str>;
    let val;
    if let Some(servername) = servername {
        let res = pgclient.query(query_string, &[&servername])?;

        if res.is_empty() {
            return Ok(());
        }

        val = res[0].get::<&str, std::net::IpAddr>("ip").to_string();
        serverip = Some(&val);
    } else {
        serverip = ip;
    }

    let serverauth = generate_serverauth(pgclient, serverip);

    for auth in serverauth {
        println!("==> {}@{} <==\n", &auth.sshuser.user, &auth.serverip);
        for key in &auth.sshuser.authorized_keys.keys {
            println!("{}", key);
        }
        println!();
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::fn_params_excessive_bools)]
#[allow(clippy::too_many_lines)]
pub fn list_servergroup(
    pgclient: &mut Client,
    servergroup: Option<&str>,
    server: Option<&str>,
    ip: Option<&str>,
    exact: bool,
    all: bool,
    empty: bool,
    json: bool,
) -> Result<(), Error> {
    let query_string = if server.is_some() || ip.is_some() || servergroup.is_some() || empty || json
    {
        r#"SELECT DISTINCT servergroup.name AS servergroup,
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
                    server.name"#
    } else {
        r#"SELECT DISTINCT servergroup.name AS servergroup,
                           NULL AS member,
                           NULL::INET AS ip,
                           servergroup.comment,
                           CASE
                               WHEN (subgroup_id IS NOT NULL) THEN 'yes'
                           END AS subgroups
           FROM servergroup
           LEFT JOIN servergroup_servergroup ON id = supergroup_id
           ORDER BY servergroup"#
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

    if let Some(servername) = server {
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

        table.set_titles(row!["servergroup", "member", "ip", "comment", "subgroups"]);

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

#[allow(clippy::too_many_lines)]
pub fn list_usergroup(
    pgclient: &mut Client,
    name: Option<&str>,
    email: Option<&str>,
    exact: bool,
    empty: bool,
    json: bool,
) -> Result<(), Error> {
    let query_string = if email.is_some() || name.is_some() || empty || json {
        r#"SELECT DISTINCT usergroup.name AS usergroup,
                           "user".email AS member,
                           usergroup.comment,
                           CASE
                               WHEN (supergroup_id IS NOT NULL) THEN
                                      (WITH RECURSIVE subgroups(id, PATH) AS
                                         (SELECT subgroup_id,
                                            (SELECT name
                                             FROM usergroup
                                             WHERE usergroup.id = subgroup_id)::TEXT AS PATH
                                          FROM usergroup_usergroup
                                          WHERE supergroup_id = usergroup.id
                                          UNION SELECT u.subgroup_id,
                                                       CONCAT(
                                                                (SELECT name
                                                                 FROM usergroup
                                                                 WHERE usergroup.id = supergroup_id), ' <- ',
                                                                (SELECT name
                                                                 FROM usergroup
                                                                 WHERE usergroup.id = subgroup_id))
                                          FROM usergroup_usergroup u
                                          JOIN subgroups x ON x.id = u.supergroup_id) SELECT string_agg(PATH, ', ')
                                       FROM subgroups
                                       WHERE id = subgroup_id)
                           END AS subgroups
           FROM usergroup
           LEFT JOIN user_usergroup AS utug
           JOIN "user" ON utug.user_id = "user".id ON usergroup.id = utug.usergroup_id
           OR usergroup.id IN
             (WITH RECURSIVE subgroups AS
                (SELECT supergroup_id
                 FROM usergroup_usergroup
                 WHERE subgroup_id = utug.usergroup_id
                 UNION SELECT u.supergroup_id
                 FROM usergroup_usergroup u
                 JOIN subgroups x ON x.supergroup_id = u.subgroup_id) SELECT DISTINCT supergroup_id
              FROM subgroups)
           LEFT JOIN usergroup_usergroup ON subgroup_id = usergroup_id
           ORDER BY usergroup,
                    "user".email"#
    } else {
        r#"SELECT DISTINCT usergroup.name AS usergroup,
                           NULL AS member,
                           usergroup.comment,
                           CASE
                               WHEN (subgroup_id IS NOT NULL) THEN 'yes'
                           END AS subgroups
           FROM usergroup
           LEFT JOIN usergroup_usergroup ON id = supergroup_id
           ORDER BY usergroup"#
    };

    let mut res = Vec::new();

    for row in pgclient.query(query_string, &[])? {
        res.push(UserGroupQuery {
            usergroup: row.get("usergroup"),
            member: row.get("member"),
            comment: row.get("comment"),
            subgroups: row.get("subgroups"),
        });
    }

    if empty {
        res.retain(|x| x.member.is_none());
    }

    if let Some(email) = email {
        res.retain(|x| match &x.member {
            Some(x) => {
                if exact {
                    x.eq(&email)
                } else {
                    x.to_lowercase().contains(&email.to_lowercase())
                }
            }
            _ => false,
        });
    }

    if let Some(name) = name {
        if exact {
            res.retain(|x| x.usergroup.eq(&name));
        } else {
            res.retain(|x| x.usergroup.to_lowercase().contains(&name.to_lowercase()));
        }
    }

    if res.is_empty() {
        return Ok(());
    }

    if json {
        println!("{}", serde_json::to_string(&res).unwrap_or_default());
    } else {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        table.set_titles(row!["usergroup", "member", "comment", "subgroups"]);

        for r in res {
            table.add_row(row![
                r.usergroup,
                r.member.unwrap_or_else(|| "-".to_string()),
                r.comment.unwrap_or_else(|| "-".to_string()),
                r.subgroups.unwrap_or_else(|| "-".to_string()),
            ]);
        }

        table.printstd();
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub fn list_useraccess(
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
    json: bool,
) -> Result<(), Error> {
    let mut query_string = if email.is_some()
        || servername.is_some()
        || ip.is_some()
        || user.is_some()
        || serveraccess.is_some()
        || servergroup.is_some()
        || usergroup.is_some()
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

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub fn list_serveraccess(
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
        r#"SELECT serveraccess.name,
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
                    server.name"#
    } else {
        // only show individual names *or* group names (w/o listing every member)
        r#"SELECT sa.name,
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
                    sshuser"#
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
