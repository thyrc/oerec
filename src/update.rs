#![allow(clippy::module_name_repetitions)]

use log::info;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, Error};

use crate::{ask_for, exit_with_message, gen_ssh_fingerprint, set_or_ask_for, ListObject};

use crate::logging::get_ssh_client;

use crate::schema::Usertype;

#[allow(clippy::too_many_lines)]
pub fn update_server(
    pgclient: &mut Client,
    serverid: Option<&str>,
    servername: Option<&str>,
    ip: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Update server");
    let query_string = r#"UPDATE server
                          SET name = $1,
                              ip = $2,
                              comment = $3
                          WHERE id = $4"#;

    let newserverid = ask_for(
        &ListObject::ServerName,
        serverid,
        Some("Server ID ['?' list by name]"),
        pgclient,
    );

    if newserverid.eq("") {
        exit_with_message("Server ID cannot be empty.");
    }

    let newserveridint = match newserverid.parse::<i64>() {
        Ok(i) => i,
        Err(_) => exit_with_message("Wrong server ID format."),
    };

    let res = pgclient.query(
        r#"SELECT name, ip, comment FROM server WHERE id = $1 LIMIT 1"#,
        &[&newserveridint],
    )?;

    if res.is_empty() {
        exit_with_message("Server not found.");
    }

    let oldservername: String = res[0].get("name");
    let oldserverip: std::net::IpAddr = res[0].get("ip");
    let oldservercomment: Option<String> = res[0].get("comment");

    let mut newservername = ask_for(
        &ListObject::ServerName,
        servername,
        Some(&format!(
            "New server name [<Enter>: '{}']",
            &oldservername.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        )),
        pgclient,
    );

    if newservername.to_lowercase().eq("") {
        newservername = oldservername.clone();
    } else if !pgclient
        .query(
            r#"SELECT id FROM server WHERE name = $1 AND id != $2"#,
            &[&newservername, &newserveridint],
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
            r#"SELECT ip FROM server WHERE ip = $1 AND id != $2"#,
            &[&newipaddr, &newserveridint],
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
        &[&newservername, &newipaddr, &newcommentopt, &newserveridint],
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

pub fn update_servergroup(
    pgclient: &mut Client,
    servergroup: Option<&str>,
    newservergroup: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Update server group");
    let query_string = r#"UPDATE servergroup
                          SET name = $1,
                              comment = $2
                          WHERE id = $3"#;

    let servergroupname = ask_for(&ListObject::ServerGroup, servergroup, None, pgclient);

    if servergroupname.eq("") {
        exit_with_message("Server group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id, name, comment FROM servergroup WHERE name = $1 LIMIT 1"#,
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
        newservergroupname = oldservergroupname.clone();
    } else if !pgclient
        .query(
            r#"SELECT id FROM servergroup WHERE name = $1 AND id != $2"#,
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

#[allow(clippy::too_many_lines)]
pub fn update_user(
    pgclient: &mut Client,
    userid: Option<&str>,
    email: Option<&str>,
    name: Option<&str>,
    usertype: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Update user");
    let query_string = r#"UPDATE "user"
           SET email = $1,
               name = $2,
               type = $3,
               comment = $4
           WHERE id = $5"#;

    let newuserid = ask_for(
        &ListObject::UserEmail,
        userid,
        Some("User ID ['?' list by email]"),
        pgclient,
    );

    if newuserid.eq("") {
        exit_with_message("User ID cannot be empty.");
    }

    let newuseridint = match newuserid.parse::<i64>() {
        Ok(i) => i,
        Err(_) => exit_with_message("Wrong user ID format."),
    };

    let res = pgclient.query(
        r#"SELECT name, email, type, comment FROM "user" WHERE id = $1 LIMIT 1"#,
        &[&newuseridint],
    )?;

    if res.is_empty() {
        exit_with_message("User not found.");
    }

    let oldusername: String = res[0].get("name");
    let olduseremail: String = res[0].get("email");
    let oldusertype: Usertype = res[0].get("type");
    let oldusercomment: Option<String> = res[0].get("comment");

    let mut newuseremail = ask_for(
        &ListObject::UserEmail,
        email,
        Some(&format!(
            "New user email ['?' for list, <Enter>: '{}']",
            &olduseremail.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        )),
        pgclient,
    );

    if newuseremail.to_lowercase().eq("") {
        newuseremail = olduseremail.clone();
    } else if !pgclient
        .query(
            r#"SELECT id FROM "user" WHERE email = $1 AND id != $2"#,
            &[&newuseremail, &newuseridint],
        )?
        .is_empty()
    {
        exit_with_message("User email already in use.");
    }

    let mut newusername = ask_for(
        &ListObject::UserName,
        name,
        Some(&format!(
            "New user name ['?' for list, <Enter>: '{}']",
            &oldusername.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        )),
        pgclient,
    );

    if newusername.to_lowercase().eq("") {
        newusername = oldusername.clone();
    } else if !pgclient
        .query(
            r#"SELECT id FROM "user" WHERE name = $1 AND id != $2"#,
            &[&newusername, &newuseridint],
        )?
        .is_empty()
    {
        exit_with_message("User name already in use.");
    }

    let oldusertypestr: String = {
        let res = pgclient.query(
            r#"SELECT type::VARCHAR FROM "user" WHERE id = $1 LIMIT 1"#,
            &[&newuseridint],
        )?;
        res[0].get("type")
    };

    let utype_prompt = set_or_ask_for(
        usertype,
        &format!(
            "New type [AD user/tool user/external user, <Enter>: '{}']",
            &oldusertypestr.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    let newutype = match utype_prompt.trim().to_lowercase().as_str() {
        "" => oldusertype,
        "ad" | "ad user" => Usertype::AD,
        "tool" | "tool user" => Usertype::Tool,
        "external" | "external user" => Usertype::External,
        _ => exit_with_message("Invalid user type."),
    };

    let newcomment = set_or_ask_for(
        comment,
        &format!(
            "New comment [<Enter>: '{}']",
            oldusercomment
                .as_ref()
                .map_or_else(|| "-".to_string(), std::string::ToString::to_string)
                .if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    let newcommentopt = if newcomment.to_lowercase().eq("") {
        oldusercomment
    } else if newcomment.trim().to_lowercase().eq("null") {
        None
    } else {
        Some(newcomment)
    };

    pgclient.query(
        query_string,
        &[
            &newuseremail,
            &newusername,
            &newutype,
            &newcommentopt,
            &newuseridint,
        ],
    )?;

    info!(
        "({}) Updated user '{}' ({}) -> '{}' ({})",
        &get_ssh_client(),
        &olduseremail,
        &oldusername,
        &newuseremail,
        &newusername
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, |t| t
            .if_supports_color(Stdout, owo_colors::OwoColorize::green))
    );

    Ok(())
}

pub fn update_usergroup(
    pgclient: &mut Client,
    usergroup: Option<&str>,
    newusergroup: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Update user group");
    let query_string = r#"UPDATE usergroup
                          SET name = $1,
                              comment = $2
                          WHERE id = $3"#;

    let usergroupname = ask_for(&ListObject::UserGroup, usergroup, None, pgclient);

    if usergroupname.eq("") {
        exit_with_message("User group name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id, name, comment FROM usergroup WHERE name = $1 LIMIT 1"#,
        &[&usergroupname],
    )?;

    if res.is_empty() {
        exit_with_message("User group not found.");
    }

    let oldusergroupid: i64 = res[0].get("id");
    let oldusergroupname: String = res[0].get("name");
    let oldusergroupcomment: Option<String> = res[0].get("comment");

    let mut newusergroupname = ask_for(
        &ListObject::UserGroup,
        newusergroup,
        Some(&format!(
            "New user group name [<Enter>: '{}']",
            oldusergroupname.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        )),
        pgclient,
    );

    if newusergroupname.to_lowercase().eq("") {
        newusergroupname = oldusergroupname.clone();
    } else if !pgclient
        .query(
            r#"SELECT id FROM usergroup WHERE name = $1 AND id != $2"#,
            &[&newusergroupname, &oldusergroupid],
        )?
        .is_empty()
    {
        exit_with_message("User group name already in use.");
    }

    let newcomment = set_or_ask_for(
        comment,
        &format!(
            "New comment [<Enter>: '{}']",
            oldusergroupcomment
                .as_ref()
                .map_or_else(|| "-".to_string(), std::string::ToString::to_string)
                .if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    let newcommentopt = if newcomment.to_lowercase().eq("") {
        oldusergroupcomment
    } else if newcomment.trim().to_lowercase().eq("null") {
        None
    } else {
        Some(newcomment)
    };

    pgclient.query(
        query_string,
        &[&newusergroupname, &newcommentopt, &oldusergroupid],
    )?;

    info!(
        "({}) Updated user group '{}' -> '{}'",
        &get_ssh_client(),
        &oldusergroupname,
        &newusergroupname
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, |t| t
            .if_supports_color(Stdout, owo_colors::OwoColorize::green))
    );

    Ok(())
}

pub fn update_key(
    pgclient: &mut Client,
    keyid: Option<&str>,
    publickey: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Update SSH key");
    let query_string = r#"UPDATE sshkeys
                          SET sshkey = $1,
                              fingerprint = $2,
                              comment = $3
                          WHERE id = $4"#;

    let newkeyid = ask_for(
        &ListObject::KeyID,
        keyid,
        Some("Key ID ['?' list by email"),
        pgclient,
    );

    if newkeyid.eq("") {
        exit_with_message("Key ID cannot be empty.");
    }

    let newkeyidint = match newkeyid.parse::<i64>() {
        Ok(i) => i,
        Err(_) => exit_with_message("Wrong key ID format."),
    };

    let res = pgclient.query(
        r#"SELECT sshkey, comment FROM sshkeys WHERE id = $1 LIMIT 1"#,
        &[&newkeyidint],
    )?;

    if res.is_empty() {
        exit_with_message("Key not found.");
    }

    let oldkey: String = res[0].get("sshkey");
    let oldkeycomment: Option<String> = res[0].get("comment");

    let mut newkey = set_or_ask_for(publickey, "New public SSH key: [<Enter>: no change]");

    if newkey.eq("") {
        newkey = oldkey.clone();
    }

    if newkey.split(' ').count() < 2 {
        exit_with_message("Invalid key format.")
    }

    newkey = newkey.split(' ').collect::<Vec<&str>>()[..2].join(" ");
    let fingerprint = &gen_ssh_fingerprint(&newkey);

    let newcomment = set_or_ask_for(
        comment,
        &format!(
            "New comment [<Enter>: '{}']",
            oldkeycomment
                .as_ref()
                .map_or_else(|| "-".to_string(), std::string::ToString::to_string)
                .if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    let newcommentopt = if newcomment.to_lowercase().eq("") {
        oldkeycomment
    } else if newcomment.trim().to_lowercase().eq("null") {
        None
    } else {
        Some(newcomment)
    };

    pgclient.query(
        query_string,
        &[&newkey, &fingerprint, &newcommentopt, &newkeyidint],
    )?;

    info!(
        "({}) Updated SSH key '{}' -> '{}'",
        &get_ssh_client(),
        &gen_ssh_fingerprint(&oldkey),
        &fingerprint
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, |t| t
            .if_supports_color(Stdout, owo_colors::OwoColorize::green))
    );

    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub fn update_serveraccess(
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
    let query_string = r#"UPDATE serveraccess
                          SET name = $1,
                              sshuser = $2,
                              sshfrom = $3,
                              sshcommand = $4,
                              sshoption = $5,
                              comment = $6,
                              server_id = $7,
                              servergroup_id = $8
                          WHERE id = $9"#;

    let newserveraccess = ask_for(&ListObject::ServerAccess, serveraccess, None, pgclient);

    if newserveraccess.eq("") {
        exit_with_message("Server access name cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id,
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
           LIMIT 1"#,
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
        newserveraccessname = oldserveraccessname.clone();
    }

    let mut newsshuser = set_or_ask_for(
        sshuser,
        &format!(
            "New SSH user [<Enter>: '{}'])",
            &oldserveraccesssshuser.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        ),
    );

    if newsshuser.eq("") {
        newsshuser = oldserveraccesssshuser.clone();
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
            r#"SELECT name FROM server WHERE id = $1 LIMIT 1"#,
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
            newservername = oldserveraccessservername.clone();
            oldserveraccessserverid
        }
        "null" => None,
        _ => {
            let res = pgclient.query(
                r#"SELECT id FROM server WHERE name = $1 LIMIT 1"#,
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
            r#"SELECT name FROM servergroup WHERE id = $1 LIMIT 1"#,
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
                newgroupname = oldserveraccessservergroupname.clone();
                oldserveraccessservergroupid
            }
            "null" => {
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
