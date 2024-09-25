use log::info;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::types::{FromSql, ToSql};
use postgres::{Client, Error};
use prettytable::{format, Table};
use serde_derive::Serialize;
use std::io::{self, Write};

use crate::logging::get_ssh_client;
use crate::{ask_for, exit_with_message, key, set_or_ask_for, user, ListObject};

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "usertype")]
enum Usertype {
    #[postgres(name = "AD user")]
    AD,
    #[postgres(name = "tool user")]
    Tool,
    #[postgres(name = "external user")]
    External,
}

#[derive(Debug, Serialize)]
struct UserQuery {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub usertype: String,
    pub disabled: String,
    pub comment: Option<String>,
}

pub fn add(
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
        key::add(pgclient, Some(&newemail), None, None)?;
    }

    Ok(())
}

pub fn add_to_usergroup(
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
        r"SELECT id FROM usergroup WHERE name = $1 LIMIT 1",
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
            user::add_to_usergroup(pgclient, None, Some(&newname))?;
        }
    }

    Ok(())
}

pub fn delete(pgclient: &mut Client, email: Option<&str>, force: bool) -> Result<(), Error> {
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

#[allow(clippy::too_many_lines)]
pub fn delete_from_usergroup(
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
            r"SELECT name FROM usergroup WHERE name = $1 LIMIT 1",
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

        if crate::usergroup::delete(pgclient, Some(&oldusergroup), true).is_err() {
            exit_with_message("Could not delete user group.");
        };
    }

    Ok(())
}

pub fn disable(pgclient: &mut Client, email: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Disable user");
    let query_string = r#"UPDATE "user" SET disabled = true WHERE email = $1"#;

    let oldemail = ask_for(&ListObject::UserEmail, email, None, pgclient);

    if oldemail.eq("") {
        exit_with_message("User email cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT name FROM "user" WHERE email = $1 LIMIT 1"#,
        &[&oldemail],
    )?;

    if res.is_empty() {
        exit_with_message("User not found.");
    }

    if !force {
        println!();
        print!(
            "Do you really want to disable user '{}'? [y/N]: ",
            &oldemail
        );
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    _ = pgclient.execute(query_string, &[&oldemail])?;

    info!("({}) Disabled user '{}'", &get_ssh_client(), &oldemail);

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn enable(pgclient: &mut Client, email: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Enable user");
    let query_string = r#"UPDATE "user" SET disabled = false WHERE email = $1"#;

    let oldemail = ask_for(&ListObject::UserEmail, email, None, pgclient);

    if oldemail.eq("") {
        exit_with_message("User email cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT name FROM "user" WHERE email = $1 LIMIT 1"#,
        &[&oldemail],
    )?;

    if res.is_empty() {
        exit_with_message("User not found.");
    }

    if !force {
        println!();
        print!("Do you really want to enable user '{}'? [y/N]: ", &oldemail);
        let mut userinput = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut userinput).unwrap();
        if !userinput.trim().to_lowercase().eq("y") {
            println!("Operation cancelled.");
            std::process::exit(1);
        }
    }

    _ = pgclient.execute(query_string, &[&oldemail])?;

    info!("({}) Enabled user '{}'", &get_ssh_client(), &oldemail);

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn list(
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

        table.set_titles(row!["email", "name", "type", "disabled", "comment"]);

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

#[allow(clippy::too_many_lines)]
pub fn update(
    pgclient: &mut Client,
    email: Option<&str>,
    newemail: Option<&str>,
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
           WHERE email = $5"#;

    let olduseremail = ask_for(&ListObject::UserEmail, email, None, pgclient);

    if olduseremail.eq("") {
        exit_with_message("User email cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id, name, type, comment FROM "user" WHERE email = $1 LIMIT 1"#,
        &[&olduseremail],
    )?;

    if res.is_empty() {
        exit_with_message("User not found.");
    }

    let olduserid: i64 = res[0].get("id");
    let oldusername: String = res[0].get("name");
    let oldusertype: Usertype = res[0].get("type");
    let oldusercomment: Option<String> = res[0].get("comment");

    let mut newuseremail = ask_for(
        &ListObject::UserEmail,
        newemail,
        Some(&format!(
            "New user email ['?' for list, <Enter>: '{}']",
            &olduseremail.if_supports_color(Stdout, owo_colors::OwoColorize::green)
        )),
        pgclient,
    );

    if newuseremail.to_lowercase().eq("") {
        newuseremail.clone_from(&olduseremail);
    } else if !pgclient
        .query(
            r#"SELECT id FROM "user" WHERE email = $1 AND id != $2"#,
            &[&newuseremail, &olduserid],
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
        newusername.clone_from(&oldusername);
    } else if !pgclient
        .query(
            r#"SELECT id FROM "user" WHERE name = $1 AND id != $2"#,
            &[&newusername, &olduserid],
        )?
        .is_empty()
    {
        exit_with_message("User name already in use.");
    }

    let oldusertypestr: String = {
        let res = pgclient.query(
            r#"SELECT type::VARCHAR FROM "user" WHERE id = $1 LIMIT 1"#,
            &[&olduserid],
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
            &olduseremail,
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
