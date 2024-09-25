use crate::exit_with_message;

pub fn add(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut serveraccess: Option<String> = None;
    let mut sshuser: Option<String> = None;
    let mut sshfrom: Option<String> = None;
    let mut sshcommand: Option<String> = None;
    let mut sshoption: Option<String> = None;
    let mut servername: Option<String> = None;
    let mut servergroup: Option<String> = None;
    let mut comment: Option<String> = None;

    let help = "oerec-add-serveraccess
Add server access

Add access to server or server group.
You'll have to specify either --server *or* --servergroup.

Usage: oerec add-serveraccess [OPTIONS] [--server <SERVER> | --servergroup <SERVERGROUP>]

Options:
        --serveraccess <NAME>          Server access name [alias: --name]
        --sshuser <SSHUSER>               SSH / OS user [aliases: --user, --osuser]

        --sshfrom <SSHFROM>            from= pattern-list
        --sshcommand <SSHCOMMAND>      command= pattern
        --sshoption <SSHOPTION>        Additional key options (`man 8 sshd`)

        --server <SERVER>              Server name
        --servergroup <SERVERGROUP>    Server group name
        --comment <COMMENT>

    -h, --help                         Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("serveraccess" | "name") => {
                serveraccess = Some(parser.value()?.string()?);
            }
            Long("sshuser" | "user" | "osuser") => {
                sshuser = Some(parser.value()?.string()?);
            }
            Long("sshfrom") => {
                sshfrom = Some(parser.value()?.string()?);
            }
            Long("sshcommand") => {
                sshcommand = Some(parser.value()?.string()?);
            }
            Long("sshoption") => {
                sshoption = Some(parser.value()?.string()?);
            }
            Long("server" | "servername") => {
                servername = Some(parser.value()?.string()?);
            }
            Long("servergroup") => {
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

    if crate::serveraccess::add(
        con,
        serveraccess.as_deref(),
        sshuser.as_deref(),
        sshfrom.as_deref(),
        sshcommand.as_deref(),
        sshoption.as_deref(),
        servername.as_deref(),
        servergroup.as_deref(),
        comment.as_deref(),
    )
    .is_err()
    {
        exit_with_message("Could not add server access.");
    };

    Ok(())
}

pub fn delete(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut serveraccess: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-delete-serveraccess
Delete server access

Usage: oerec delete-serveraccess [OPTIONS]

Options:
        --serveraccess <SERVERACCESS>    [alias: --name]
        --confirm                        Skip confirmation dialog

    -h, --help                           Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("serveraccess" | "name") => {
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

    if crate::serveraccess::delete(con, serveraccess.as_deref(), confirm).is_err() {
        exit_with_message("Could not delete server access.");
    };

    Ok(())
}

pub fn list(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut server: Option<String> = None;
    let mut ip: Option<String> = None;
    let mut sshuser: Option<String> = None;
    let mut serveraccess: Option<String> = None;
    let mut servergroup: Option<String> = None;
    let mut exact: bool = false;
    let mut json: bool = false;

    let help = "oerec-list-serveracess
List server access

Usage: oerec list-serveraccess [OPTIONS]

Options:
        --server <SERVERNAME>          List server access on server SERVERNAME
        --ip <IP>                      List server access on server w/ IP
        --sshuser <SSHUSER>            List server access w/ SSHUSER user [aliases: --user, --osuser]

    -e, --exact                        Only list exact matches
    -j, --json                         Set output mode to JSON

    -h, --help                         Print this message

Filter:
        --serveraccess <NAME>          Filter output by NAME [alias: --name]
        --servergroup <SERVERGROUP>    Filter output by SERVERGROUP";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server" | "servername") => {
                server = Some(parser.value()?.string()?);
            }
            Long("ip") => {
                ip = Some(parser.value()?.string()?);
            }
            Long("sshuser" | "user" | "osuser") => {
                sshuser = Some(parser.value()?.string()?);
            }
            Long("serveraccess" | "name") => {
                serveraccess = Some(parser.value()?.string()?);
            }
            Long("servergroup") => {
                servergroup = Some(parser.value()?.string()?);
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

    if crate::serveraccess::list(
        con,
        serveraccess.as_deref(),
        server.as_deref(),
        ip.as_deref(),
        sshuser.as_deref(),
        servergroup.as_deref(),
        exact,
        json,
    )
    .is_err()
    {
        exit_with_message("Could not list server access.");
    };

    Ok(())
}

pub fn update(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut serveraccess: Option<String> = None;
    let mut newserveraccess: Option<String> = None;
    let mut sshuser: Option<String> = None;
    let mut sshfrom: Option<String> = None;
    let mut sshcommand: Option<String> = None;
    let mut sshoption: Option<String> = None;
    let mut servername: Option<String> = None;
    let mut servergroup: Option<String> = None;
    let mut comment: Option<String> = None;

    let help = "oerec-update-serveraccess
Update server access

Usage: oerec update-serveraccess [OPTIONS]

Options:
        --serveraccess <NAME>          Server access name [alias: --name]
        --newname <NEWNAME>            New server access name [alias: --newserveraccess]
        --sshuser <SSHUSER>            SSH / OS user [alias: --user]

        --sshfrom <SSHFROM>            from= pattern-list
        --sshcommand <SSHCOMMAND>      command= pattern
        --sshoption <SSHOPTION>        additional options, e.g `no-pty`

        --server <SERVER>              Server name
        --servergroup <SERVERGROUP>    Server group name
        --comment <COMMENT>

    -h, --help                         Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("serveraccess" | "name") => {
                serveraccess = Some(parser.value()?.string()?);
            }
            Long("newname" | "newserveraccess") => {
                newserveraccess = Some(parser.value()?.string()?);
            }
            Long("sshuser" | "user" | "osuser") => {
                sshuser = Some(parser.value()?.string()?);
            }
            Long("sshfrom") => {
                sshfrom = Some(parser.value()?.string()?);
            }
            Long("sshcommand") => {
                sshcommand = Some(parser.value()?.string()?);
            }
            Long("sshoption") => {
                sshoption = Some(parser.value()?.string()?);
            }
            Long("server" | "servername") => {
                servername = Some(parser.value()?.string()?);
            }
            Long("servergroup") => {
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

    if crate::serveraccess::update(
        con,
        serveraccess.as_deref(),
        newserveraccess.as_deref(),
        sshuser.as_deref(),
        sshfrom.as_deref(),
        sshcommand.as_deref(),
        sshoption.as_deref(),
        servername.as_deref(),
        servergroup.as_deref(),
        comment.as_deref(),
    )
    .is_err()
    {
        exit_with_message("Could not update server access.");
    };

    Ok(())
}
