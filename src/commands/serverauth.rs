use std::ffi::OsString;

use crate::exit_with_message;

pub fn list(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut server: Option<String> = None;
    let mut ip: Option<String> = None;

    let help = "oerec-list-serverauth
List server auth

Usage: oerec list-serverauth [OPTIONS]

Options:
        --server <SERVERNAME>    List server auth by SERVERNAME (only exact matches)
        --ip <IP>                List server auth by IP

    -h, --help                   Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("server") => {
                server = Some(parser.value()?.string()?);
            }
            Long("ip") => {
                ip = Some(parser.value()?.string()?);
            }
            Long("help") | Short('h') => {
                println!("{help}");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if crate::serverauth::list(con, ip.as_deref(), server.as_deref()).is_err() {
        exit_with_message("Could not list server auth.");
    };

    Ok(())
}

pub fn write(con: &mut postgres::Client, parser: &mut lexopt::Parser) -> Result<(), lexopt::Error> {
    use lexopt::prelude::*;

    let mut workdir: Option<OsString> = None;
    let mut force: bool = false;

    let help = "oerec-write-serverauth
Write authorized_keys to workdir

Usage: oerec write-serverauth [OPTIONS] --workdir <WORKDIR>

Options:
        --workdir <WORKDIR>    [alias: --dir]
        --force                Overwrite workdir contents (USE WITH CAUTION)

    -h, --help                 Print this message";

    while let Some(arg) = parser.next()? {
        match arg {
            Long("workdir" | "dir") => {
                workdir = Some(parser.value()?.parse()?);
            }
            Long("force") => {
                force = true;
            }
            Long("help") | Short('h') => {
                println!("{help}");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    crate::serverauth::write(con, workdir.as_deref(), force);

    Ok(())
}
