use log::info;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, Error};
use prettytable::{format, Table};
use serde_derive::Serialize;
use std::io::{self, Write};

use crate::logging::get_ssh_client;
use crate::{ask_for, exit_with_message, set_or_ask_for, user, ListObject};

#[derive(Debug, Serialize)]
struct UserGroupQuery {
    pub usergroup: String,
    pub member: Option<String>,
    pub comment: Option<String>,
    pub subgroups: Option<String>,
}

pub fn add(
    pgclient: &mut Client,
    usergroup: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Add user group");
    let query_string = r"INSERT INTO usergroup (name, comment) VALUES ($1, $2)";

    let newname = ask_for(&ListObject::UserGroup, usergroup, None, pgclient);

    if newname.eq("") {
        exit_with_message("Group name cannot be empty.");
    }

    let res = pgclient.query(
        r"SELECT id FROM usergroup WHERE name = $1 LIMIT 1",
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
            user::add_to_usergroup(pgclient, None, Some(&newname))?;
        }
    }

    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub fn add_to_usergroup(
    pgclient: &mut Client,
    subgroup: Option<&str>,
    supergroup: Option<&str>,
) -> Result<(), Error> {
    println!("Add user group to user group");
    let query_string = r"INSERT INTO usergroup_usergroup (subgroup_id, supergroup_id)
                          SELECT ug1.id,
                            (SELECT ug2.id
                             FROM usergroup ug2
                             WHERE ug2.name = $2)
                          FROM usergroup ug1
                          WHERE ug1.name = $1";

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
        r"SELECT id FROM usergroup WHERE name = $1 LIMIT 1",
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
        r"SELECT id FROM usergroup WHERE name = $1 LIMIT 1",
        &[&newsupergroupname],
    )?;

    if res.is_empty() {
        exit_with_message("User group not found.");
    }

    let res = pgclient.query(
        r"SELECT subgroup_id, supergroup_id
           FROM usergroup_usergroup
           JOIN usergroup AS ug1 ON ug1.id = supergroup_id
           JOIN usergroup AS ug2 ON ug2.id = subgroup_id
           WHERE ug1.name = $1
             AND ug2.name = $2",
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
            add_to_usergroup(pgclient, None, Some(&newsupergroupname))?;
        }
    }

    Ok(())
}

pub fn delete(pgclient: &mut Client, usergroup: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Delete user group");
    let query_string = r"DELETE FROM usergroup WHERE name = $1";

    let oldusergroup = ask_for(&ListObject::UserGroup, usergroup, None, pgclient);

    if oldusergroup.eq("") {
        exit_with_message("User group cannot be empty.");
    }

    if pgclient
        .query(
            r"SELECT name FROM usergroup WHERE name = $1 LIMIT 1",
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

#[allow(clippy::module_name_repetitions)]
pub fn delete_from_usergroup(
    pgclient: &mut Client,
    subgroup: Option<&str>,
    supergroup: Option<&str>,
    force: bool,
) -> Result<(), Error> {
    println!("Delete user group from user group");
    let query_string = r"DELETE
                          FROM usergroup_usergroup
                          WHERE (subgroup_id,
                                 supergroup_id) =
                            (SELECT subgroup_id,
                                    supergroup_id
                             FROM usergroup_usergroup
                             JOIN usergroup AS ug1 ON ug1.id = subgroup_id
                             JOIN usergroup AS ug2 ON ug2.id = supergroup_id
                             WHERE ug1.name = $1
                               AND ug2.name = $2)";

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
        r"SELECT id FROM usergroup WHERE name = $1 LIMIT 1",
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
        r"SELECT id FROM usergroup WHERE name = $1 LIMIT 1",
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

#[allow(clippy::too_many_lines)]
pub fn list(
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
        r"SELECT DISTINCT usergroup.name AS usergroup,
                           NULL AS member,
                           usergroup.comment,
                           CASE
                               WHEN (subgroup_id IS NOT NULL) THEN 'yes'
                           END AS subgroups
           FROM usergroup
           LEFT JOIN usergroup_usergroup ON id = supergroup_id
           ORDER BY usergroup"
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

pub fn update(
    pgclient: &mut Client,
    usergroup: Option<&str>,
    newusergroup: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Update user group");
    let query_string = r"UPDATE usergroup
                          SET name = $1,
                              comment = $2
                          WHERE id = $3";

    let usergroupname = ask_for(&ListObject::UserGroup, usergroup, None, pgclient);

    if usergroupname.eq("") {
        exit_with_message("User group name cannot be empty.");
    }

    let res = pgclient.query(
        r"SELECT id, name, comment FROM usergroup WHERE name = $1 LIMIT 1",
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
        newusergroupname.clone_from(&oldusergroupname);
    } else if !pgclient
        .query(
            r"SELECT id FROM usergroup WHERE name = $1 AND id != $2",
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
