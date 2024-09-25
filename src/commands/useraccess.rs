use crate::exit_with_message;

pub fn add(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut email: Option<String> = None;
    let mut usergroup: Option<String> = None;
    let mut serveraccess: Option<String> = None;
    let mut until: Option<String> = None;
    let mut comment: Option<String> = None;

    let help = "oerec-add-useraccess
Add user access

Add either user (via email) *or* usergroup (via user group name) to server access.

Usage: oerec add-useraccess [OPTIONS] [ --email <EMAIL> | --usergroup <USERGROUP> ]

Options:
        --email <EMAIL>
        --usergroup <USERGROUP>          [alias: --groupname]
        --serveraccess <SERVERACCESS>
        --until <UNTIL>                  Format: YYYY-MM-DD, optional w/ HH:MI:SS
        --comment <COMMENT>

    -h, --help                           Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("email") => {
                email = Some(parser.value()?.string()?);
            }
            Long("usergroup" | "groupname") => {
                usergroup = Some(parser.value()?.string()?);
            }
            Long("serveraccess") => {
                serveraccess = Some(parser.value()?.string()?);
            }
            Long("until") => {
                until = Some(parser.value()?.string()?);
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

    if crate::useraccess::add(
        con,
        email.as_deref(),
        usergroup.as_deref(),
        serveraccess.as_deref(),
        until.as_deref(),
        comment.as_deref(),
    )
    .is_err()
    {
        exit_with_message("Could not add user access.");
    };

    Ok(())
}

pub fn delete(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut email: Option<String> = None;
    let mut usergroup: Option<String> = None;
    let mut serveraccess: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-delete-useraccess
Delete user access

Usage: oerec delete-useraccess [OPTIONS]

Options:
        --email <EMAIL>
        --usergroup <USERGROUP>          [alias: --groupname]
        --serveraccess <SERVERACCESS> 
        --confirm                        Skip confirmation dialog

    -h, --help                           Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("email") => {
                email = Some(parser.value()?.string()?);
            }
            Long("usergroup" | "groupname") => {
                usergroup = Some(parser.value()?.string()?);
            }
            Long("serveraccess") => {
                serveraccess = Some(parser.value()?.string()?);
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

    if crate::useraccess::delete(
        con,
        email.as_deref(),
        usergroup.as_deref(),
        serveraccess.as_deref(),
        confirm,
    )
    .is_err()
    {
        exit_with_message("Could not delete user access.");
    };

    Ok(())
}

pub fn list(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut server: Option<String> = None;
    let mut ip: Option<String> = None;
    let mut email: Option<String> = None;
    let mut sshuser: Option<String> = None;
    let mut serveraccess: Option<String> = None;
    let mut servergroup: Option<String> = None;
    let mut usergroup: Option<String> = None;
    let mut show_expired: bool = false;
    let mut show_disabled: bool = false;
    let mut exact: bool = false;
    let mut json: bool = false;

    let help = "oerec-list-useraccess
List user access

Usage: oerec list-useraccess [OPTIONS]

Options:
        --server <SERVERNAME>           List user access on server SERVERNAME
        --ip <IP>                       List user access on server w/ IP
        --email <EMAIL>                 List useraccess containing member w/ EMAIL
        --sshuser <USER>                List user access w/ SSH USER [aliases: --user, --osuser]
        --serveraccess <SERVERACCESS>   List user / user group w/ access to SERVERACCESS

        --expired                       List only expired useraccess entries
        --disabled                      Only show disabled (user *or* server) entries
    -e, --exact                         Only list exact matches
    -j, --json                          Set output mode to JSON

    -h, --help                          Print this message

Filter:
        --servergroup <SERVERGROUP>     Filter output by server group
        --usergroup <USERGROUP>         Filter output by user group";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server" | "servername") => {
                server = Some(parser.value()?.string()?);
            }
            Long("ip") => {
                ip = Some(parser.value()?.string()?);
            }
            Long("email") => {
                email = Some(parser.value()?.string()?);
            }
            Long("sshuser" | "user" | "osuser") => {
                sshuser = Some(parser.value()?.string()?);
            }
            Long("serveraccess") => {
                serveraccess = Some(parser.value()?.string()?);
            }
            Long("servergroup") => {
                servergroup = Some(parser.value()?.string()?);
            }
            Long("usergroup") => {
                usergroup = Some(parser.value()?.string()?);
            }
            Long("expired") => {
                show_expired = true;
            }
            Long("disabled") => {
                show_disabled = true;
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

    if crate::useraccess::list(
        con,
        email.as_deref(),
        server.as_deref(),
        ip.as_deref(),
        sshuser.as_deref(),
        serveraccess.as_deref(),
        servergroup.as_deref(),
        usergroup.as_deref(),
        exact,
        show_expired,
        show_disabled,
        json,
    )
    .is_err()
    {
        exit_with_message("Could not list user access.");
    };

    Ok(())
}
