use crate::exit_with_message;

pub fn add(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut usergroup: Option<String> = None;
    let mut comment: Option<String> = None;

    let help = "oerec-add-usergroup
Add user group

Usage: oerec add-usergroup [OPTIONS]

Options:
        --usergroup <NAME>     Group name [alias: --groupname]
        --comment <COMMENT>

    -h, --help                 Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("usergroup" | "groupname") => {
                usergroup = Some(parser.value()?.string()?);
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

    if crate::usergroup::add(con, usergroup.as_deref(), comment.as_deref()).is_err() {
        exit_with_message("Could not add user group.");
    };

    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub fn add_to_usergroup(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut subgroup: Option<String> = None;
    let mut supergroup: Option<String> = None;

    let help = "oerec-add-usergroup-to-usergroup
Add user group to user group

Usage: oerec add-usergroup-to-usergroup [OPTIONS]

Options:
        --subgroup <SUBGROUP>        Member user group name
        --supergroup <SUPERGROUP>    Parent user group name

    -h, --help                       Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("subgroup" | "childgroup" | "member") => {
                subgroup = Some(parser.value()?.string()?);
            }
            Long("supergroup" | "parentgroup") => {
                supergroup = Some(parser.value()?.string()?);
            }
            Long("help") | Short('h') => {
                println!("{help}");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if crate::usergroup::add_to_usergroup(con, subgroup.as_deref(), supergroup.as_deref()).is_err()
    {
        exit_with_message("Could not add user group to user group.");
    };

    Ok(())
}

pub fn delete(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut usergroup: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-delete-usergroup
Delete user group

Usage: oerec delete-usergroup [OPTIONS]

Options:
        --usergroup <USERGROUP>    [alias: --groupname]
        --confirm                  Skip confirmation dialog

    -h, --help                     Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
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

    if crate::usergroup::delete(con, usergroup.as_deref(), confirm).is_err() {
        exit_with_message("Could not delete user group.");
    };

    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub fn delete_from_usergroup(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut subgroup: Option<String> = None;
    let mut supergroup: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-delete-usergroup-from-usergroup
Delete user group from user group

Usage: oerec delete-usergroup-from-usergroup [OPTIONS]

Options:
        --subgroup <SUBGROUP>        Member user group name
        --supergroup <SUPERGROUP>    Parent user group name
        --confirm                    Skip confirmation dialog

    -h, --help                       Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("subgroup" | "childgroup" | "member") => {
                subgroup = Some(parser.value()?.string()?);
            }
            Long("supergroup" | "parentgroup") => {
                supergroup = Some(parser.value()?.string()?);
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

    if crate::usergroup::delete_from_usergroup(
        con,
        subgroup.as_deref(),
        supergroup.as_deref(),
        confirm,
    )
    .is_err()
    {
        exit_with_message("Could not delte user group from user group.");
    };

    Ok(())
}

pub fn list(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut email: Option<String> = None;
    let mut usergroup: Option<String> = None;
    let mut show_empty: bool = false;
    let mut exact: bool = false;
    let mut json: bool = false;

    let help = "oerec-list-usergroup
List user groups

Usage: oerec list-usergroup [OPTIONS]

Options:
        --email <EMAIL>       List user group containing member w/ EMAIL
        --usergroup <NAME>    List user group w/ NAME [alias: --groupname]

        --empty               List only user groups w/o members
    -e, --exact               Only list exact matches
    -j, --json                Set output mode to JSON

    -h, --help                Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("email") => {
                email = Some(parser.value()?.string()?);
            }
            Long("usergroup" | "groupname") => {
                usergroup = Some(parser.value()?.string()?);
            }
            Long("empty") => {
                show_empty = true;
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

    if crate::usergroup::list(
        con,
        usergroup.as_deref(),
        email.as_deref(),
        exact,
        show_empty,
        json,
    )
    .is_err()
    {
        exit_with_message("Could not list user group.");
    };

    Ok(())
}

pub fn update(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut usergroup: Option<String> = None;
    let mut newusergroup: Option<String> = None;
    let mut comment: Option<String> = None;

    let help = "oerec-update-usergroup
Update user group

Usage: oerec update-usergroup [OPTIONS]

Options:
        --usergroup <USERGROUP>    [alias: --groupname]
        --newname <NEWNAME>        New group name
        --comment <COMMENT>

    -h, --help                     Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("usergroup" | "groupname") => {
                usergroup = Some(parser.value()?.string()?);
            }
            Long("newname") => {
                newusergroup = Some(parser.value()?.string()?);
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

    if crate::usergroup::update(
        con,
        usergroup.as_deref(),
        newusergroup.as_deref(),
        comment.as_deref(),
    )
    .is_err()
    {
        exit_with_message("Could not update user group.");
    };

    Ok(())
}

//                 .about("Write authorized_keys to workdir")
//                 .override_help("oerec-write-serverauth
// Write authorized_keys to workdir
//
// Usage: oerec write-serverauth [OPTIONS] --workdir <WORKDIR>
//
// Options:
//         --workdir <WORKDIR>    [alias: --dir]
//         --force                Overwrite workdir contents (USE WITH CAUTION)
//
//     -h, --help                 Print this message")
//                 .display_order(600)
//                 .arg(
//                     Arg::new("WORKDIR")
//                         .long("workdir")
//                         .visible_alias("dir")
//                         .required(true)
//                         .value_parser(ValueParser::os_string())
//                         .num_args(1),
//                 )
//                 .arg(
//                     Arg::new("FORCE")
//                         .long("force")
//                         .action(ArgAction::SetTrue)
//                         .help("Overwrite workdir contents (USE WITH CAUTION)")
//                 ),
//         ])
// }
