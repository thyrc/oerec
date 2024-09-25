use crate::exit_with_message;

pub fn add(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servername: Option<String> = None;
    let mut ip: Option<String> = None;
    let mut comment: Option<String> = None;
    let mut disabled: bool = false;
    let mut enable_dns: bool = false;

    let help = "oerec-add-server
Add server

Usage: oerec add-server [OPTIONS]

Options:
        --server <NAME>        Server name [alias: --name]
        --ip <IP>
        --comment <COMMENT>

        --disabled             Add server but disable key distribution
        --enable-dns           Resolve server name to determine target IP

    -h, --help                 Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server" | "name") => {
                servername = Some(parser.value()?.string()?);
            }
            Long("ip") => {
                ip = Some(parser.value()?.string()?);
            }
            Long("comment") => {
                comment = Some(parser.value()?.string()?);
            }
            Long("disabled") => {
                disabled = true;
            }
            Long("enable-dns") => {
                enable_dns = true;
            }
            Long("help") | Short('h') => {
                println!("{help}");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if crate::server::add(
        con,
        servername.as_deref(),
        ip.as_deref(),
        disabled,
        enable_dns,
        comment.as_deref(),
    )
    .is_err()
    {
        exit_with_message("Could not add server.");
    };

    Ok(())
}

pub fn add_to_servergroup(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servergroup: Option<String> = None;
    let mut servername: Option<String> = None;

    let help = "oerec-add-server-to-servergroup
Add server to server group

Usage: oerec add-server-to-servergroup [OPTIONS]

Options:
        --servergroup <NAME>    Group name [alias: --groupname]
        --server <SERVER>       Server name

    -h, --help                  Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("servergroup" | "groupname") => {
                servergroup = Some(parser.value()?.string()?);
            }
            Long("server" | "servername") => {
                servername = Some(parser.value()?.string()?);
            }
            Long("help") | Short('h') => {
                println!("{help}");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if crate::server::add_to_servergroup(con, servername.as_deref(), servergroup.as_deref())
        .is_err()
    {
        exit_with_message("Could not add server to server group.");
    };

    Ok(())
}

pub fn delete(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servername: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-delete-server
Delete server

Usage: oerec delete-server [OPTIONS]

Options:
        --server <SERVER>    [alias: --name]
        --confirm            Skip confirmation dialog

    -h, --help               Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server" | "name") => {
                servername = Some(parser.value()?.string()?);
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

    if crate::server::delete(con, servername.as_deref(), confirm).is_err() {
        exit_with_message("Could not delete server.");
    };

    Ok(())
}

pub fn disable(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servername: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-disable-server
Disable server

Usage: oerec disable-server [OPTIONS]

Options:
        --server <NAME>    [alias: --name]
        --confirm          Skip confirmation dialog

    -h, --help             Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server" | "name") => {
                servername = Some(parser.value()?.string()?);
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

    if crate::server::disable(con, servername.as_deref(), confirm).is_err() {
        exit_with_message("Could not disable server.");
    };

    Ok(())
}

pub fn disable_dns(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servername: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-disable-dns
Disable server DNS lookup

Usage: oerec disable-dns [OPTIONS]

Options:
        --server <NAME>    [alias: --servername]
        --confirm          Skip confirmation dialog

    -h, --help             Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server" | "servername") => {
                servername = Some(parser.value()?.string()?);
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

    if crate::server::disable_dns(con, servername.as_deref(), confirm).is_err() {
        exit_with_message("Could not disable server.");
    };

    Ok(())
}

pub fn delete_from_servergroup(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servername: Option<String> = None;
    let mut servergroup: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-delete-server-from-servergroup
Delete server from server group

Usage: oerec delete-server-from-servergroup [OPTIONS]

Options:
        --server <SERVER>              Server Name
        --servergroup <SERVERGROUP>    Group name [alias: --groupname]
        --confirm                      Skip confirmation dialog

    -h, --help                         Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server" | "servername" | "name") => {
                servername = Some(parser.value()?.string()?);
            }
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

    if crate::server::delete_from_servergroup(
        con,
        servername.as_deref(),
        servergroup.as_deref(),
        confirm,
    )
    .is_err()
    {
        exit_with_message("Could not delete server from server group.");
    };

    Ok(())
}

pub fn enable(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servername: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-enable-server
Enable server

Usage: oerec enable-server [OPTIONS]

Options:
        --server <NAME>    [aliases: name]
        --confirm          Skip confirmation dialog

    -h, --help             Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server" | "name") => {
                servername = Some(parser.value()?.string()?);
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

    if crate::server::enable(con, servername.as_deref(), confirm).is_err() {
        exit_with_message("Could not enable server.");
    };

    Ok(())
}

pub fn enable_dns(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servername: Option<String> = None;
    let mut confirm: bool = false;

    let help = "oerec-enable-dns
Enable server DNS lookup

Usage: oerec enable-dns [OPTIONS]

Options:
        --server <NAME>    [alias: --servername]
        --confirm          Skip confirmation dialog

    -h, --help             Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server" | "servername") => {
                servername = Some(parser.value()?.string()?);
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

    if crate::server::enable_dns(con, servername.as_deref(), confirm).is_err() {
        exit_with_message("Could not enable server.");
    };

    Ok(())
}

pub fn list(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut server: Option<String> = None;
    let mut ip: Option<String> = None;
    let mut serverid: Option<String> = None;
    let mut exact: bool = false;
    let mut json: bool = false;

    let help = "oerec-list-server
List managed servers

Usage: oerec list-server [OPTIONS]

Options:
        --server <SERVERNAME>    List server by SERVERNAME [alias: --name]
        --ip <IP>                List server by IP
        --id <ID>                List server w/ ID

    -e, --exact                  Only list exact matches
    -j, --json                   Set output mode to JSON

    -h, --help                   Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server" | "name") => {
                server = Some(parser.value()?.string()?);
            }
            Long("ip") => {
                ip = Some(parser.value()?.string()?);
            }
            Long("id" | "serverid") => {
                serverid = Some(parser.value()?.string()?);
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

    if crate::server::list(
        con,
        server.as_deref(),
        ip.as_deref(),
        serverid.as_deref(),
        exact,
        json,
    )
    .is_err()
    {
        exit_with_message("Could not list server.");
    };

    Ok(())
}

pub fn update(
    con: &mut postgres::Client,
    parser: &mut lexopt::Parser,
) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut servername: Option<String> = None;
    let mut newservername: Option<String> = None;
    let mut ip: Option<String> = None;
    let mut comment: Option<String> = None;

    let help = "oerec-update-server
Update server

Usage: oerec update-server [OPTIONS]

Options:
        --server <NAME>        Server name [alias: --name]
        --newname <NAME>       New server name
        --ip <IP>
        --comment <COMMENT>

    -h, --help                 Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server" | "name") => {
                servername = Some(parser.value()?.string()?);
            }
            Long("newname") => {
                newservername = Some(parser.value()?.string()?);
            }
            Long("ip") => {
                ip = Some(parser.value()?.string()?);
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

    if crate::server::update(
        con,
        servername.as_deref(),
        newservername.as_deref(),
        ip.as_deref(),
        comment.as_deref(),
    )
    .is_err()
    {
        exit_with_message("Could not update server.");
    };

    Ok(())
}
