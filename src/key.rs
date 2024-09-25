use base64::{engine, Engine};
use log::info;
use owo_colors::{OwoColorize, Stream::Stdout};
use postgres::{Client, Error};
use prettytable::{format, Table};
use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use std::io::{self, Write};

use crate::logging::get_ssh_client;
use crate::{ask_for, exit_with_message, set_or_ask_for, ListObject};

#[derive(Debug, Serialize)]
struct SshKeysQuery {
    pub id: i64,
    pub email: String,
    pub sshkey: String,
    pub fingerprint: String,
    pub comment: Option<String>,
}

pub fn add(
    pgclient: &mut Client,
    email: Option<&str>,
    publickey: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Add SSH key");
    let query_string = r#"INSERT INTO sshkeys (user_id, sshkey, fingerprint, comment)
                          SELECT "user".id,
                                 $1,
                                 $2,
                                 $3
                          FROM "user"
                          WHERE "user".email = $4"#;

    let newemail = ask_for(&ListObject::UserEmail, email, None, pgclient);

    if newemail.eq("") {
        exit_with_message("User email cannot be empty.");
    }

    let res = pgclient.query(
        r#"SELECT id FROM "user" WHERE email = $1 LIMIT 1"#,
        &[&newemail],
    )?;

    if res.is_empty() {
        exit_with_message("User not found.")
    }

    let mut newkey = set_or_ask_for(publickey, "Public SSH key");

    if newkey.eq("") {
        exit_with_message("Key cannot be empty.");
    }

    if newkey.split(' ').count() < 2 {
        exit_with_message("Invalid key format.")
    }

    newkey = newkey.split(' ').collect::<Vec<&str>>()[..2].join(" ");
    let fingerprint = &generate_fingerprint(&newkey);

    let newcomment = set_or_ask_for(comment, "Comment");
    let newcomment = match &newcomment.trim().to_lowercase()[..] {
        "" | "null" => None,
        _ => Some(newcomment),
    };

    pgclient.query(
        query_string,
        &[&newkey, &fingerprint, &newcomment, &newemail],
    )?;

    info!(
        "({}) Added SSH key '{}' for user '{}'",
        &get_ssh_client(),
        &fingerprint,
        &newemail
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, owo_colors::OwoColorize::green)
    );

    Ok(())
}

pub fn delete(pgclient: &mut Client, keyid: Option<&str>, force: bool) -> Result<(), Error> {
    println!("Delete public SSH key");
    let query_string = r"DELETE FROM sshkeys WHERE id = $1";

    let oldkeyid = ask_for(
        &ListObject::KeyID,
        keyid,
        Some("Key ID ['?' list by email"),
        pgclient,
    );

    if oldkeyid.eq("") {
        exit_with_message("Key ID cannot be empty.");
    }

    let Ok(keyint) = oldkeyid.parse::<i64>() else {
        exit_with_message("Wrong key ID format.")
    };

    let fingerprint = pgclient.query(
        r"SELECT fingerprint FROM sshkeys WHERE id = $1 LIMIT 1",
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

pub fn list(
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
        let Ok(idint) = id.parse::<i64>() else {
            exit_with_message("Wrong ID format.")
        };
        res.retain(|x| x.id.eq(&idint));
    }

    if res.is_empty() {
        return Ok(());
    }

    #[allow(clippy::uninlined_format_args)]
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

fn generate_fingerprint(key: &str) -> String {
    let sshkey = key.split(' ').collect::<Vec<&str>>();

    if sshkey.len() < 2 {
        exit_with_message("Wrong SSH key format.");
    }

    if sshkey[0].eq("ssh-ed25519") || sshkey[0].eq("ssh-rsa") || sshkey[0].eq("ecdsa-sha2-nistp256")
    {
        if let Ok(binkey) = engine::general_purpose::STANDARD.decode(sshkey[1]) {
            let mut hasher = Sha256::new();
            hasher.update(binkey);
            let result = hasher.finalize();
            let mut fingerprint = String::from("SHA256:");
            fingerprint.push_str(&engine::general_purpose::STANDARD_NO_PAD.encode(result));
            return fingerprint;
        }
    }

    exit_with_message("Wrong SSH key format.");
}

pub fn update(
    pgclient: &mut Client,
    keyid: Option<&str>,
    publickey: Option<&str>,
    comment: Option<&str>,
) -> Result<(), Error> {
    println!("Update SSH key");
    let query_string = r"UPDATE sshkeys
                          SET sshkey = $1,
                              fingerprint = $2,
                              comment = $3
                          WHERE id = $4";

    let newkeyid = ask_for(
        &ListObject::KeyID,
        keyid,
        Some("Key ID ['?' list by email"),
        pgclient,
    );

    if newkeyid.eq("") {
        exit_with_message("Key ID cannot be empty.");
    }

    let Ok(newkeyidint) = newkeyid.parse::<i64>() else {
        exit_with_message("Wrong key ID format.")
    };

    let res = pgclient.query(
        r"SELECT sshkey, comment FROM sshkeys WHERE id = $1 LIMIT 1",
        &[&newkeyidint],
    )?;

    if res.is_empty() {
        exit_with_message("Key not found.");
    }

    let oldkey: String = res[0].get("sshkey");
    let oldkeycomment: Option<String> = res[0].get("comment");

    let mut newkey = set_or_ask_for(publickey, "New public SSH key: [<Enter>: no change]");

    if newkey.eq("") {
        newkey.clone_from(&oldkey);
    }

    if newkey.split(' ').count() < 2 {
        exit_with_message("Invalid key format.")
    }

    newkey = newkey.split(' ').collect::<Vec<&str>>()[..2].join(" ");
    let fingerprint = &generate_fingerprint(&newkey);

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
        &generate_fingerprint(&oldkey),
        &fingerprint
    );

    println!(
        "{}",
        "Done.".if_supports_color(Stdout, |t| t
            .if_supports_color(Stdout, owo_colors::OwoColorize::green))
    );

    Ok(())
}
