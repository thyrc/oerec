use crate::exit_with_message;

pub fn add(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servergroup: Option<String> = None;
    let mut comment: Option<String> = None;

    let help = "oerec-add-servergroup
Add server group

Usage: oerec add-servergroup [OPTIONS]

Options:
        --servergroup <NAME>     Group name [alias: --groupname]
        --comment <COMMENT>

    -h, --help                 Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("servergroup" | "groupname") => {
                servergroup = Some(parser.value()?.string()?);
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

    if crate::servergroup::add(con, servergroup.as_deref(), comment.as_deref()).is_err() {
        exit_with_message("Could not add server group.");
    };

    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub fn add_to_servergroup(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut subgroup: Option<String> = None;
    let mut supergroup: Option<String> = None;

    let help = "oerec-add-servergroup-to-servergroup
Add server group to server group

Usage: oerec add-servergroup-to-servergroup [OPTIONS]

Options:
        --subgroup <SUBGROUP>        Member server group name
        --supergroup <SUPERGROUP>    Parent server group name

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

    if crate::servergroup::add_to_servergroup(con, subgroup.as_deref(), supergroup.as_deref())
        .is_err()
    {
        exit_with_message("Could not add server group to server group.");
    };

    Ok(())
}

pub fn delete(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servergroup: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-delete-servergroup
Delete server group

Usage: oerec delete-servergroup [OPTIONS]

Options:
        --servergroup <SERVERGROUP>    [alias: --groupname]
        --confirm                      Skip confirmation dialog

    -h, --help                         Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("servergroup" | "groupname") => {
                servergroup = Some(parser.value()?.string()?);
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

    if crate::servergroup::delete(con, servergroup.as_deref(), confirm).is_err() {
        exit_with_message("Could not delete server group.");
    };

    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub fn delete_from_servergroup(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut subgroup: Option<String> = None;
    let mut supergroup: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-delete-servergroup-from-servergroup
Delete server group from server group

Usage: oerec delete-servergroup-from-servergroup [OPTIONS]

Options:
        --subgroup <SUBGROUP>        Member server group name
        --supergroup <SUPERGROUP>    Parent server group name
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

    if crate::servergroup::delete_from_servergroup(
        con,
        subgroup.as_deref(),
        supergroup.as_deref(),
        confirm,
    )
    .is_err()
    {
        exit_with_message("Could not delete server group from server group.");
    };

    Ok(())
}

pub fn list(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servergroup: Option<String> = None;
    let mut server: Option<String> = None;
    let mut ip: Option<String> = None;
    let mut show_all: bool = false;
    let mut show_empty: bool = false;
    let mut exact: bool = false;
    let mut json: bool = false;

    let help = "oerec-list-servergroup
List server groups

Usage: oerec list-servergroup [OPTIONS]

Options:
        --server <SERVERNAME>          List server group containing server w/ SERVERNAME
        --ip <IP>                      List server group containing server w/ IP

        --all                          include the 'all' server group
        --empty                        List only server groups w/o members
    -e, --exact                        Only list exact matches
    -j, --json                         Set output mode to JSON

    -h, --help                         Print this message

Filter:
        --servergroup <SERVERGROUP>    Filter output by group name [alias: --groupname]";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server" | "servername") => {
                server = Some(parser.value()?.string()?);
            }
            Long("ip") => {
                ip = Some(parser.value()?.string()?);
            }
            Long("servergroup" | "groupname") => {
                servergroup = Some(parser.value()?.string()?);
            }
            Long("all") => {
                show_all = true;
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

    if crate::servergroup::list(
        con,
        servergroup.as_deref(),
        server.as_deref(),
        ip.as_deref(),
        exact,
        show_all,
        show_empty,
        json,
    )
    .is_err()
    {
        exit_with_message("Could not list server group.");
    };

    Ok(())
}

pub fn update(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servergroup: Option<String> = None;
    let mut newservergroup: Option<String> = None;
    let mut comment: Option<String> = None;

    let help = "oerec-update-servergroup
Update server group

Usage: oerec update-servergroup [OPTIONS]

Options:
        --servergroup <SERVERGROUP>     [alias: --groupname]
        --newname <NEWNAME>             New group name
        --comment <COMMENT>

    -h, --help                          Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("servergroup" | "groupname") => {
                servergroup = Some(parser.value()?.string()?);
            }
            Long("newname") => {
                newservergroup = Some(parser.value()?.string()?);
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

    if crate::servergroup::update(
        con,
        servergroup.as_deref(),
        newservergroup.as_deref(),
        comment.as_deref(),
    )
    .is_err()
    {
        exit_with_message("Could not update server group.");
    };

    Ok(())
}
