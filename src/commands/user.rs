use crate::exit_with_message;

pub fn add(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut email: Option<String> = None;
    let mut name: Option<String> = None;
    let mut comment: Option<String> = None;
    let mut usertype: Option<String> = None;

    let help = "oerec-add-user
Add user

Usage: oerec add-user [OPTIONS]

Options:
        --email <EMAIL>
        --name <NAME>
        --type <TYPE>          `AD user` / `tool user` / `external user`
        --comment <COMMENT>

    -h, --help                 Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("email") => {
                email = Some(parser.value()?.string()?);
            }
            Long("name" | "username") => {
                name = Some(parser.value()?.string()?);
            }
            Long("type") => {
                usertype = Some(parser.value()?.string()?);
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

    if crate::user::add(
        con,
        email.as_deref(),
        name.as_deref(),
        usertype.as_deref(),
        comment.as_deref(),
    )
    .is_err()
    {
        exit_with_message("Could not add user.");
    };

    Ok(())
}

pub fn add_to_usergroup(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut usergroup: Option<String> = None;
    let mut email: Option<String> = None;

    let help = "oerec-add-user-to-usergroup
Add user to user group

Usage: oerec add-user-to-usergroup [OPTIONS]

Options:
        --usergroup <NAME>    Group name [alias: --groupname]
        --email <EMAIL>

    -h, --help                Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("usergroup" | "groupname") => {
                usergroup = Some(parser.value()?.string()?);
            }
            Long("email") => {
                email = Some(parser.value()?.string()?);
            }
            Long("help") | Short('h') => {
                println!("{help}");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if crate::user::add_to_usergroup(con, email.as_deref(), usergroup.as_deref()).is_err() {
        exit_with_message("Could not add user to user group.");
    };

    Ok(())
}

pub fn delete(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut email: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-delete-user
Delete user

Usage: oerec delete-user [OPTIONS]

Options:
        --email <EMAIL>
        --confirm          Skip confirmation dialog

    -h, --help             Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("email") => {
                email = Some(parser.value()?.string()?);
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

    if crate::user::delete(con, email.as_deref(), confirm).is_err() {
        exit_with_message("Could not delete user.");
    };

    Ok(())
}

pub fn disable(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut email: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-disable-user
Disable user

Usage: oerec disable-user [OPTIONS]

Options:
        --email <EMAIL>
        --confirm          Skip confirmation dialog

    -h, --help             Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("email") => {
                email = Some(parser.value()?.string()?);
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

    if crate::user::disable(con, email.as_deref(), confirm).is_err() {
        exit_with_message("Could not disable user.");
    };

    Ok(())
}

pub fn delete_from_usergroup(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut email: Option<String> = None;
    let mut usergroup: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-delete-user-from-usergroup
Delete user from user group

Usage: oerec delete-user-from-usergroup [OPTIONS]

Options:
        --email <EMAIL>
        --usergroup <USERGROUP>    Group name [alias: --groupname]
        --confirm                  Skip confirmation dialog

    -h, --help                     Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("email") => {
                email = Some(parser.value()?.string()?);
            }
            Long("usergroup" | "groupname") => {
                usergroup = Some(parser.value()?.string()?);
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

    if crate::user::delete_from_usergroup(con, email.as_deref(), usergroup.as_deref(), confirm)
        .is_err()
    {
        exit_with_message("Could not delete user from user group.");
    };

    Ok(())
}

pub fn enable(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut email: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-enable-user
Enable user

Usage: oerec enable-user [OPTIONS]

Options:
        --email <EMAIL>
        --confirm          Skip confirmation dialog

    -h, --help             Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("email") => {
                email = Some(parser.value()?.string()?);
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

    if crate::user::enable(con, email.as_deref(), confirm).is_err() {
        exit_with_message("Could not enable user.");
    };

    Ok(())
}

pub fn list(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut email: Option<String> = None;
    let mut userid: Option<String> = None;
    let mut name: Option<String> = None;
    let mut exact: bool = false;
    let mut json: bool = false;

    let help = "oerec-list-user
List user accounts

Usage: oerec list-user [OPTIONS]

Options:
        --email <EMAIL>    List user by EMAIL
        --name <NAME>      List user by NAME
        --id <ID>          List user w/ ID

    -e, --exact            Only list exact matches
    -j, --json             Set output mode to JSON

    -h, --help             Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("email") => {
                email = Some(parser.value()?.string()?);
            }
            Long("name") => {
                name = Some(parser.value()?.string()?);
            }
            Long("id" | "userid") => {
                userid = Some(parser.value()?.string()?);
            }
            Long("exact") | Short('e') => {
                exact = true;
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

    if crate::user::list(
        con,
        email.as_deref(),
        name.as_deref(),
        userid.as_deref(),
        exact,
        json,
    )
    .is_err()
    {
        exit_with_message("Could not list user.");
    };

    Ok(())
}

pub fn update(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut email: Option<String> = None;
    let mut newemail: Option<String> = None;
    let mut name: Option<String> = None;
    let mut usertype: Option<String> = None;
    let mut comment: Option<String> = None;

    let help = "oerec-update-user
Update user

Usage: oerec update-user [OPTIONS]

Options:
        --email <EMAIL>        User email
        --newemail <EMAIL>     New user email
        --name <NAME>
        --type <TYPE>          `AD user` / `tool user` / `external user`
        --comment <COMMENT>

    -h, --help                 Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("email") => {
                email = Some(parser.value()?.string()?);
            }
            Long("newemail") => {
                newemail = Some(parser.value()?.string()?);
            }
            Long("name" | "username") => {
                name = Some(parser.value()?.string()?);
            }
            Long("type") => {
                usertype = Some(parser.value()?.string()?);
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

    if crate::user::update(
        con,
        email.as_deref(),
        newemail.as_deref(),
        name.as_deref(),
        usertype.as_deref(),
        comment.as_deref(),
    )
    .is_err()
    {
        exit_with_message("Could not update user.");
    };

    Ok(())
}
