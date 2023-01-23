#![allow(clippy::module_name_repetitions)]
use log::{error, warn};
use postgres::Client;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::net::{IpAddr, ToSocketAddrs};

use crate::exit_with_message;

#[derive(Debug, Serialize)]
pub struct AuthorizedKeys {
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthorizedUser {
    pub user: String,
    pub authorized_keys: AuthorizedKeys,
}

#[derive(Debug, Serialize)]
pub struct ServerAuth {
    pub serverip: String,
    pub sshuser: AuthorizedUser,
}

#[derive(Debug)]
struct AuthQuery {
    _userid: i64,
    _keyid: i64,
    host: std::net::IpAddr,
    sshuser: String,
    sshfrom: Option<String>,
    sshcommand: Option<String>,
    sshoption: Option<String>,
    sshkey: String,
    email: String,
    comment: Option<String>,
}

#[allow(clippy::too_many_lines)]
pub fn generate_serverauth(pgclient: &mut Client, ip: Option<&str>) -> Vec<ServerAuth> {
    let dns_enabled = match pgclient.query(
        "SELECT ip, name FROM server WHERE use_dns AND NOT disabled",
        &[],
    ) {
        Ok(x) => x,
        Err(_) => exit_with_message("Could not generate resolve list."),
    };

    let update_query = r#"UPDATE server SET ip = $1 WHERE name = $2"#;

    for name in dns_enabled {
        let mut n = name.get::<&str, String>("name");
        n.push_str(":80");
        let addrs = if let Ok(x) = n.to_socket_addrs() {
            x
        } else {
            eprintln!(
                "(DNS Query) No IP address found for '{}'",
                &n.trim_end_matches(":80")
            );
            warn!(
                "(DNS Query) No IP address found for '{}'",
                &n.trim_end_matches(":80")
            );
            continue;
        };

        let iplist = addrs.map(|x| x.ip()).collect::<Vec<IpAddr>>();

        if !iplist.contains(&name.get::<&str, IpAddr>("ip")) {
            if pgclient
                .query(update_query, &[&iplist[0], &n.trim_end_matches(":80")])
                .is_ok()
            {
                warn!(
                    "(DNS Query) Updated IP for '{}' ({} -> {})",
                    &n.trim_end_matches(":80"),
                    &name.get::<&str, IpAddr>("ip"),
                    &iplist[0]
                );
            } else {
                eprintln!(
                    "(DNS Query) Could not update IP for '{}' ({} -> {})",
                    &n.trim_end_matches(":80"),
                    &name.get::<&str, IpAddr>("ip"),
                    &iplist[0]
                );
                error!(
                    "(DNS Query) Could not update IP for '{}' ({} -> {})",
                    &n.trim_end_matches(":80"),
                    &name.get::<&str, IpAddr>("ip"),
                    &iplist[0]
                );
            }
        }
    }

    // Built with simplicity in mind... not performance :D
    let auth_query = r#"SELECT DISTINCT "user".id AS userid,
                                        sshkeys.id AS keyid,
                                        server.ip,
                                        serveraccess.sshuser,
                                        serveraccess.sshfrom,
                                        serveraccess.sshcommand,
                                        serveraccess.sshoption,
                                        sshkeys.sshkey,
                                        "user".email,
                                        SUBSTRING(sshkeys.comment, 1, 64) AS COMMENT
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
                                         INNER JOIN subgroups x ON x.supergroup_id = u.subgroup_id) SELECT DISTINCT supergroup_id
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
                        JOIN sshkeys ON "user".id = sshkeys.user_id
                        WHERE NOT "user".disabled
                          AND useraccess.best_before > NOW()
                          AND NOT server.disabled
                        ORDER BY "user".id,
                                 sshkeys.id"#;

    let mut serverauth: Vec<ServerAuth> = Vec::new();

    let res = match pgclient.query(auth_query, &[]) {
        Ok(x) => x,
        Err(_) => exit_with_message("Could not generate auth list."),
    };

    let mut hm = HashMap::new();

    for row in res {
        let auth: AuthQuery = AuthQuery {
            _userid: row.get("userid"),
            _keyid: row.get("keyid"),
            host: row.get("ip"),
            sshuser: row.get("sshuser"),
            sshfrom: row.get("sshfrom"),
            sshcommand: row.get("sshcommand"),
            sshoption: row.get("sshoption"),
            sshkey: row.get("sshkey"),
            email: row.get("email"),
            comment: row.get("comment"),
        };

        let mut l = String::new();

        // sshfrom
        if let Some(from) = &auth.sshfrom {
            l.push_str(r#"from=""#);
            l.push_str(from);
            l.push('"');
            if auth.sshcommand.is_some() || auth.sshoption.is_some() {
                l.push(',');
            } else {
                l.push(' ');
            }
        }

        // sshcommand
        if let Some(command) = &auth.sshcommand {
            l.push_str(r#"command=""#);
            l.push_str(command);
            l.push('"');
            if auth.sshoption.is_some() {
                l.push(',');
            } else {
                l.push(' ');
            }
        }

        // sshoption
        if let Some(option) = &auth.sshoption {
            l.push_str(option);
            l.push(' ');
        }

        // key
        l.push_str(&auth.sshkey);
        l.push(' ');
        l.push_str(&auth.email);
        if let Some(comment) = &auth.comment {
            l.push_str(r#" ("#);
            l.push_str(comment);
            l.push(')');
        }

        hm.entry((auth.host, auth.sshuser))
            .or_insert_with(Vec::new)
            .push(l);
    }

    if let Some(ip) = ip {
        hm.retain(|k, _| k.0.to_string().contains(ip));
    }

    for (k, v) in hm {
        let (host, user) = k;

        serverauth.push(ServerAuth {
            serverip: host.to_string(),
            sshuser: AuthorizedUser {
                user: user.to_string(),
                authorized_keys: AuthorizedKeys { keys: v },
            },
        });
    }

    serverauth
}
