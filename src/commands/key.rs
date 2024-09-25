use crate::exit_with_message;

pub fn add(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut email: Option<String> = None;
    let mut sshkey: Option<String> = None;
    let mut comment: Option<String> = None;

    let help = "oerec-add-key
Add public SSH key

Usage: oerec add-key [OPTIONS]

Options:
        --email <EMAIL>
        --sshkey <KEY>
        --comment <COMMENT>

    -h, --help                 Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("email") => {
                email = Some(parser.value()?.string()?);
            }
            Long("sshkey") => {
                sshkey = Some(parser.value()?.string()?);
            }
            Long("comment") => {
                comment = Some(parser.value()?.string()?);
            }
            Long("help") | Short('h') => {
                println!("{help}");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if crate::key::add(con, email.as_deref(), sshkey.as_deref(), comment.as_deref()).is_err() {
        exit_with_message("Could not add key.");
    };

    Ok(())
}

pub fn delete(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut keyid: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-delete-key
Delete SSH key

Usage: oerec delete-key [OPTIONS]

Options:
        --id <KEYID>

        --confirm       Skip confirmation dialog

    -h, --help          Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("id" | "keyid") => {
                keyid = Some(parser.value()?.string()?);
            }
            Long("confirm") => {
                confirm = true;
            }
            Long("help") | Short('h') => {
                println!("{help}");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if crate::key::delete(con, keyid.as_deref(), confirm).is_err() {
        exit_with_message("Could not delete key.");
    };

    Ok(())
}

pub fn list(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut email: Option<String> = None;
    let mut fingerprint: Option<String> = None;
    let mut keyid: Option<String> = None;
    let mut with_key: bool = false;
    let mut json: bool = false;

    let help = "oerec-list-key
List SSH keys

Usage: oerec list-key [OPTIONS]

Options:
        --email <EMAIL>                List keys by user EMAIL
        --fingerprint <FINGERPRINT>    List keys by (SHA256) FINGERPRINT
        --id <ID>                      List key w/ ID

        --with-key                     Display public SSH keys [alias: --long]
    -j, --json                         Set output mode to JSON

    -h, --help                         Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("email") => {
                email = Some(parser.value()?.string()?);
            }
            Long("fingerprint") => {
                fingerprint = Some(parser.value()?.string()?);
            }
            Long("id" | "keyid") => {
                keyid = Some(parser.value()?.string()?);
            }
            Long("with-key" | "long") => {
                with_key = true;
            }
            Long("json") | Short('j') => {
                json = true;
            }
            Long("help") | Short('h') => {
                println!("{help}");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if crate::key::list(
        con,
        email.as_deref(),
        fingerprint.as_deref(),
        with_key,
        keyid.as_deref(),
        json,
    )
    .is_err()
    {
        exit_with_message("Could not list keys.");
    };

    Ok(())
}

pub fn update(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut keyid: Option<String> = None;
    let mut sshkey: Option<String> = None;
    let mut comment: Option<String> = None;

    let help = "oerec-update-key
Update public SSH key

Usage: oerec update-key [OPTIONS]

Options:
        --id <KEYID>           [alias: --keyid]
        --sshkey <KEY>
        --comment <COMMENT>

    -h, --help                 Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("id" | "keyid") => {
                keyid = Some(parser.value()?.string()?);
            }
            Long("sshkey") => {
                sshkey = Some(parser.value()?.string()?);
            }
            Long("comment") => {
                comment = Some(parser.value()?.string()?);
            }
            Long("help") | Short('h') => {
                println!("{help}");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if crate::key::update(con, keyid.as_deref(), sshkey.as_deref(), comment.as_deref()).is_err() {
        exit_with_message("Could not update key.");
    };

    Ok(())
}
