use clap::builder::ValueParser;
use clap::{Arg, Command};

#[allow(clippy::too_many_lines)]
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::must_use_candidate)]
pub fn build_cli() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
        .disable_colored_help(true)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("\u{d8}re client")
        .long_about("Does awesome things")
        .override_help("oerec

\u{d8}re client

USAGE:
    oerec [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help       Print this message
    -j, --json       Set output mode to JSON for commands that support it (list-*)
    -V, --version    Print version information

SUBCOMMANDS:

  server management:
    add-server, list-server, update-server, delete-server

  server group management:
    add-servergroup, list-servergroup, update-servergroup, delete-servergroup

    add-server-to-servergroup, add-servergroup-to-servergroup
    delete-server-from-servergroup, delete-servergroup-from-servergroup

  user management:
    add-user, list-user, update-user, delete-user

  key management:
    add-key, list-key, update-key, delete-key

  user group management:
    add-usergroup, list-usergroup, update-usergroup, delete-usergroup

    add-user-to-usergroup, add-usergroup-to-usergroup
    delete-user-from-usergroup, delete-usergroup-from-usergroup

  access management:
    add-serveraccess, list-serveraccess, update-serveraccess, delete-serveraccess
    add-useraccess, list-useraccess, delete-useraccess

  maintenance:
    enable-dns, disable-dns
    enable-server, disable-server
    enable-user, disable-user

  write:
    write-serverauth

  help:
    help    Print this message or the help of the given subcommand")
        .arg(
            Arg::new("JSONOUTPUT")
                .long("json")
                .short('j')
                .help("Return list-* output as JSON object"),
        ).
        subcommands(vec![
            Command::new("list-user")
                .alias("list-users")
                .about("List user accounts")
                .override_help("oerec-list-user
List user accounts

USAGE:
    oerec list-user [OPTIONS]

OPTIONS:
        --email <EMAIL>    List user by EMAIL
        --name <NAME>      List user by NAME
        --id <ID>          List user w/ ID

    -e, --exact            Only list exact matches

    -h, --help             Print this message")
                .display_order(200)
                .arg(
                    Arg::new("EMAIL")
                        .long("email")
                        .help("List user by email [LIKE %EMAIL%]")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("ID")
                        .long("id")
                        .help("List user w/ ID")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("NAME")
                        .long("name")
                        .help("List user by name [LIKE %NAME%]")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("EXACT")
                        .long("exact")
                        .short('e')
                        .help("Only list exact matches")
                ),
            Command::new("list-key")
                .alias("list-keys")
                .about("List SSH keys")
                .override_help("oerec-list-key
List SSH keys

USAGE:
    oerec list-key [OPTIONS]

OPTIONS:
        --email <EMAIL>                List keys by user EMAIL
        --fingerprint <FINGERPRINT>    List keys by (SHA256) FINGERPRINT
        --id <ID>                      List key w/ ID

        --with-key                     Display public SSH keys [alias: --long]

    -h, --help                         Print this message")
                .display_order(200)
                .arg(
                    Arg::new("EMAIL")
                        .long("email")
                        .help("List keys by user email [LIKE %EMAIL%]")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("ID")
                        .long("id")
                        .help("List key w/ ID")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FINGERPRINT")
                        .long("fingerprint")
                        .help("List keys by SHA256 fingerprint")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("WITHKEY")
                        .long("with-key")
                        .visible_alias("long")
                        .help("List public SSH keys"),
                ),
            Command::new("list-server")
                .about("List managed servers")
                .override_help("oerec-list-server
List managed servers

USAGE:
    oerec list-server [OPTIONS]

OPTIONS:
        --ip <IP>                List server by IP
        --server <SERVERNAME>    List server by SERVERNAME [alias: --name]
        --id <ID>                List server w/ ID

    -e, --exact                  Only list exact matches

    -h, --help                   Print this message")
                .display_order(200)
                .arg(
                    Arg::new("IP")
                        .long("ip")
                        .help("List server by IP [LIKE %IP%]")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("ID")
                        .long("id")
                        .help("List server w/ ID")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERNAME")
                        .long("server")
                        .visible_alias("name")
                        .help("List server by name [LIKE %SERVERNAME%]")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("EXACT")
                        .long("exact")
                        .short('e')
                        .help("Only list exact matches")
                ),
            Command::new("list-serverauth")
                .hide(true)
                .about("List server auth")
                .override_help("oerec-list-serverauth
List server auth

USAGE:
    oerec list-serverauth [OPTIONS]

OPTIONS:
        --ip <IP>                List server auth by IP
        --server <SERVERNAME>    List server auth by SERVERNAME

    -h, --help                   Print this message")
                .display_order(200)
                .arg(
                    Arg::new("IP")
                        .long("ip")
                        .help("List server auth by IP")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERNAME")
                        .long("server")
                        .help("List server auth by name")
                        .conflicts_with("IP")
                        .takes_value(true),
                ),
            Command::new("list-servergroup")
                .alias("list-servergroups")
                .about("List server groups")
                .override_help("oerec-list-servergroup
List server groups

USAGE:
    oerec list-servergroup [OPTIONS]

OPTIONS:
        --ip <IP>                      List server group containing server w/ IP
        --server <SERVERNAME>          List server group containing server w/ SERVERNAME

        --all                          include the 'all' server group
        --empty                        List only server groups w/o members
    -e, --exact                        Only list exact matches

    -h, --help                         Print this message

FILTER:
        --servergroup <SERVERGROUP>    Filter output by group name [alias: --groupname]")
                .display_order(200)
                .arg(
                    Arg::new("SERVERGROUP")
                        .long("servergroup")
                        .visible_alias("groupname")
                        .help("WARNING: This is just a filter.\nFilter server group by name\nThis will *not* generate a complete list.")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("IP")
                        .long("ip")
                        .help("List server group containing server w/ IP [LIKE %IP%]")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERNAME")
                        .long("server")
                        .help("List server group containing server w/ SERVERNAME [LIKE %SERVERNAME%]")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("ALL")
                        .long("all")
                        .help("include the 'all' server group"),
                )
                .arg(
                    Arg::new("EMPTY")
                        .long("empty")
                        .help("List only server groups w/o members"),
                )
                .arg(
                    Arg::new("EXACT")
                        .long("exact")
                        .short('e')
                        .help("Only list exact matches")
                ),
            Command::new("list-usergroup")
                .alias("list-usergroups")
                .about("List user groups")
                .override_help("oerec-list-usergroup
List user groups

USAGE:
    oerec list-usergroup [OPTIONS]

OPTIONS:
        --email <EMAIL>       List user group containing member w/ EMAIL
        --groupname <NAME>    List user group w/ NAME [alias: --usergroup]

        --empty               List only user groups w/o members
    -e, --exact               Only list exact matches

    -h, --help                Print this message")
                .display_order(200)
                .arg(
                    Arg::new("EMAIL")
                        .long("email")
                        .help("List user group containing member w/ EMAIL [LIKE %EMAIL%]")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("NAME")
                        .long("groupname")
                        .visible_alias("usergroup")
                        .help("List user group w/ name [LIKE %NAME%]")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("EMPTY")
                        .long("empty")
                        .help("List only user groups w/o members"),
                )
                .arg(
                    Arg::new("EXACT")
                        .long("exact")
                        .short('e')
                        .help("Only list exact matches")
                ),
            Command::new("list-useraccess")
                .about("List user access")
                .override_help("oerec-list-useraccess
List user access

USAGE:
    oerec list-useraccess [OPTIONS]

OPTIONS:
        --ip <IP>                       List user access on server w/ IP
        --server <SERVERNAME>           List user access on server SERVERNAME
        --email <EMAIL>                 List useraccess containing member w/ EMAIL
        --sshuser <USER>                List user access w/ SSH USER [aliases: --user, --osuser]
        --serveraccess <SERVERACCESS>   List user / user group w/ access to SERVERACCESS

        --expired                       List only expired useraccess entries
    -e, --exact                         Only list exact matches

    -h, --help                          Print this message

FILTER:
        --servergroup <SERVERGROUP>     Filter output by server group
        --usergroup <USERGROUP>         Filter output by user group")
                .display_order(200)
                .arg(
                    Arg::new("EMAIL")
                        .long("email")
                        .help("List useraccess containing member w/ EMAIL")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERNAME")
                        .long("server")
                        .help("List user access on server SERVERNAME")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("IP")
                        .long("ip")
                        .help("List user access on server w/ IP")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("USER")
                        .long("sshuser")
                        .visible_alias("user")
                        .alias("osuser")
                        .help("List user access w/ SSH USER")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERACCESS")
                        .long("serveraccess")
                        .help("List user access w/ SERVERACCESS")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERGROUP")
                        .long("servergroup")
                        .help("WARNING: This is just a filter.\nFilter user access on server group\nThis will *not* generate a complete list.")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("USERGROUP")
                        .long("usergroup")
                        .help("WARNING: This is just a filter.\nFilter server access by user group\nThis will *not* generate a complete list.")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("EXPIRED")
                        .long("expired")
                        .help("List only expired useraccess entries"),
                )
                .arg(
                Arg::new("EXACT")
                    .long("exact")
                    .short('e')
                    .help("Only list exact matches")
                ),
            Command::new("list-serveraccess")
                .about("List server access")
                .override_help("oerec-list-serveraccess
List server access

USAGE:
    oerec list-serveraccess [OPTIONS]

OPTIONS:
        --ip <IP>                      List server access on server w/ IP
        --server <SERVERNAME>          List server access on server SERVERNAME
        --user <SSHUSER>               List server access w/ SSHUSER user [alias: --sshuser]

    -e, --exact                        Only list exact matches

    -h, --help                         Print this message

FILTER:
        --serveraccess <NAME>          Filter output by NAME [alias: --name]
        --servergroup <SERVERGROUP>    Filter output by SERVERGROUP")
                .display_order(200)
                .arg(
                    Arg::new("NAME")
                        .long("serveraccess")
                        .visible_alias("name")
                        .help("WARNING: This is just a filter.\nFilter server access by name\nThis will *not* generate a complete list.")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERNAME")
                        .long("server")
                        .help("List server access on server SERVERNAME [LIKE %SERVERNAME%]")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("IP")
                        .long("ip")
                        .help("List server access on server w/ IP [LIKE %IP%]")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SSHUSER")
                        .long("user")
                        .visible_alias("sshuser")
                        .help("List server access w/ SSHUSER user [LIKE %SSHUSER")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERGROUP")
                        .long("servergroup")
                        .help("WARNING: This is just a filter.\nFilter server access by server group\nThis will *not* generate a complete list.")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("EXACT")
                        .long("exact")
                        .short('e')
                        .help("Only list exact matches")
                ),
            Command::new("add-server")
                .about("Add server")
                .override_help("oerec-add-server
Add server

USAGE:
    oerec add-server [OPTIONS]

OPTIONS:
        --ip <IP>
        --server <NAME>        Server name [alias: --name]
        --comment <COMMENT>

        --disabled             Add server but disable key distribution
        --enable-dns           Resolve server name to determine target IP

    -h, --help                 Print this message")
                .display_order(100)
                .arg(
                    Arg::new("NAME")
                        .long("server")
                        .visible_alias("name")
                        .help("Server name")
                        .takes_value(true),
                )
                .arg(Arg::new("IP").long("ip").takes_value(true))
                .arg(Arg::new("DISABLED").long("disabled").help("Add server but disable key distribution"))
                .arg(Arg::new("DNS").long("enable-dns").alias("usedns").help("Resolve server name to determine target IP"))
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("add-user")
                .about("Add user")
                .override_help("oerec-add-user
Add user

USAGE:
    oerec add-user [OPTIONS]

OPTIONS:
        --email <EMAIL>
        --name <NAME>
        --type <TYPE>          `AD user` / `tool user` / `external user`
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(100)
                .arg(Arg::new("EMAIL").long("email").takes_value(true))
                .arg(Arg::new("NAME").long("name").takes_value(true))
                .arg(Arg::new("TYPE").long("type").takes_value(true))
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("add-key")
                .alias("add-sshkey")
                .about("Add public SSH key")
                .override_help("oerec-add-key
Add public SSH key

USAGE:
    oerec add-key [OPTIONS]

OPTIONS:
        --email <EMAIL>
        --sshkey <KEY>
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(100)
                .arg(Arg::new("EMAIL").long("email").takes_value(true))
                .arg(Arg::new("KEY").long("sshkey").takes_value(true))
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("add-usergroup")
                .about("Add user group")
                .override_help("oerec-add-usergroup
Add user group

USAGE:
    oerec add-usergroup [OPTIONS]

OPTIONS:
        --usergroup <NAME>     Group name [alias: --groupname]
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(100)
                .arg(
                    Arg::new("NAME")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .help("Group name")
                        .takes_value(true),
                )
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("add-user-to-usergroup")
                .about("Add user to user group")
                .override_help("oerec-add-user-to-usergroup
Add user to user group

USAGE:
    oerec add-user-to-usergroup [OPTIONS]

OPTIONS:
        --usergroup <NAME>    Group name [alias: --groupname]
        --email <EMAIL>

    -h, --help                Print this message")
                .display_order(100)
                .arg(
                    Arg::new("NAME")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .help("Group name")
                        .takes_value(true),
                )
                .arg(Arg::new("EMAIL").long("email").takes_value(true)),
            Command::new("add-usergroup-to-usergroup")
                .about("Add user group to user group")
                .override_help("oerec-add-usergroup-to-usergroup
Add user group to user group

USAGE:
    oerec add-usergroup-to-usergroup [OPTIONS]

OPTIONS:
        --subgroup <SUBGROUP>        Member user group name
        --supergroup <SUPERGROUP>    Parent user group name

    -h, --help                       Print this message")
                .display_order(100)
                .arg(
                    Arg::new("SUBGROUP")
                        .long("subgroup")
                        .help("Member user group name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SUPERGROUP")
                        .long("supergroup")
                        .help("Parent user group name")
                        .takes_value(true),
                ),
            Command::new("add-servergroup")
                .about("Add server group")
                .override_help("oerec-add-servergroup
Add server group

USAGE:
    oerec add-servergroup [OPTIONS]

OPTIONS:
        --servergroup <NAME>     Group name [alias: --groupname]
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(100)
                .arg(
                    Arg::new("NAME")
                        .long("servergroup")
                        .visible_alias("groupname")
                        .help("Group name")
                        .takes_value(true),
                )
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("add-server-to-servergroup")
                .about("Add server to server group")
                .override_help("oerec-add-server-to-servergroup
Add server to server group

USAGE:
    oerec add-server-to-servergroup [OPTIONS]

OPTIONS:
        --servergroup <NAME>    Group name [alias: --groupname]
        --server <SERVER>       Server name

    -h, --help                  Print this message")
                .display_order(100)
                .arg(
                    Arg::new("NAME")
                        .long("servergroup")
                        .visible_alias("groupname")
                        .help("Group name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVER")
                        .long("server")
                        .help("Server name")
                        .takes_value(true),
                ),
            Command::new("add-servergroup-to-servergroup")
                .about("Add server group to server group")
                .override_help("oerec-add-servergroup-to-servergroup
Add server group to server group

USAGE:
    oerec add-servergroup-to-servergroup [OPTIONS]

OPTIONS:
        --subgroup <SUBGROUP>        Member server group name
        --supergroup <SUPERGROUP>    Parent server group name

    -h, --help                       Print this message")
                .display_order(100)
                .arg(
                    Arg::new("SUBGROUP")
                        .long("subgroup")
                        .help("Member server group name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SUPERGROUP")
                        .long("supergroup")
                        .help("Parent server group name")
                        .takes_value(true),
                ),
            Command::new("add-serveraccess")
                .about("Add server access\n\nAdd access to server or server group.\nYou'll have to specify either --server *or* --servergroup.")
                .override_help("oerec-add-serveraccess
Add server access

Add access to server or server group.
You'll have to specify either --server *or* --servergroup.

USAGE:
    oerec add-serveraccess [OPTIONS] [--server <SERVER> | --servergroup <SERVERGROUP>]

OPTIONS:
        --serveraccess <NAME>          Server access name [alias: --name]
        --user <SSHUSER>               SSH / OS user [alias: --sshuser]

        --sshfrom <SSHFROM>            from= pattern-list
        --sshcommand <SSHCOMMAND>      command= pattern
        --sshoption <SSHOPTION>        Additional key options (`man 8 sshd`)

        --server <SERVER>              Server name
        --servergroup <SERVERGROUP>    Server group name
        --comment <COMMENT>

    -h, --help                         Print this message")
                .display_order(100)
                .arg(
                    Arg::new("NAME")
                        .long("serveraccess")
                        .visible_alias("name")
                        .help("Server access name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SSHUSER")
                        .long("user")
                        .visible_alias("sshuser")
                        .help("SSH / OS user")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SSHFROM")
                        .long("sshfrom")
                        .help("from= pattern-list")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SSHCOMMAND")
                        .long("sshcommand")
                        .help("command= pattern")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SSHOPTION")
                        .long("sshoption")
                        .help("Additional key options (`man 8 sshd`)")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVER")
                        .long("server")
                        .help("Server name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERGROUP")
                        .conflicts_with("SERVER")
                        .long("servergroup")
                        .help("Server group name")
                        .takes_value(true),
                )
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("add-useraccess")
                .about("Add user access\n\nAdd either user (via email) *or* usergroup (via user group name) to server access.")
                .override_help("oerec-add-useraccess
Add user access

Add either user (via email) *or* usergroup (via user group name) to server access.

USAGE:
    oerec add-useraccess [OPTIONS] [ --email <EMAIL> | --usergroup <USERGROUP> ]

OPTIONS:
        --email <EMAIL>
        --usergroup <USERGROUP>          [alias: --groupname]
        --serveraccess <SERVERACCESS>
        --until <UNTIL>                  Format: YYYY-MM-DD, optional w/ HH:MI:SS
        --comment <COMMENT>

    -h, --help                           Print this message")
                .display_order(100)
                .arg(Arg::new("EMAIL").long("email").takes_value(true))
                .arg(
                    Arg::new("USERGROUP")
                        .conflicts_with("EMAIL")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERACCESS")
                        .long("serveraccess")
                        .takes_value(true),
                )
                .arg(Arg::new("UNTIL").long("until").help("Format: YYYY-MM-DD, optional w/ HH:MI:SS").takes_value(true))
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("delete-server")
                .about("Delete server")
                .override_help("oerec-delete-server
Delete server

USAGE:
    oerec delete-server [OPTIONS]

OPTIONS:
        --server <SERVER>
        --confirm            Skip confirmation dialog

    -h, --help               Print this message")
                .display_order(300)
                .arg(
                    Arg::new("SERVER")
                        .long("server")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-user")
                .about("Delete user")
                .override_help("oerec-delete-user
Delete user

USAGE:
    oerec delete-user [OPTIONS]

OPTIONS:
        --email <EMAIL>
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(300)
                .arg(
                    Arg::new("EMAIL")
                        .long("email")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-key")
                .about("Delete SSH key")
                .override_help("oerec-delete-key
Delete SSH key

USAGE:
    oerec delete-key [OPTIONS]

OPTIONS:
        --id <KEYID>
        --confirm       Skip confirmation dialog

    -h, --help          Print this message")
                .display_order(300)
                .arg(
                    Arg::new("KEYID")
                        .long("id")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-user-from-usergroup")
                .about("Delete user from user group")
                .override_help("oerec-delete-user-from-usergroup
Delete user from user group

USAGE:
    oerec delete-user-from-usergroup [OPTIONS]

OPTIONS:
        --email <EMAIL>
        --usergroup <USERGROUP>    Group name [alias: --groupname]
        --confirm                  Skip confirmation dialog

    -h, --help                     Print this message")
                .display_order(300)
                .arg(Arg::new("EMAIL").long("email").takes_value(true))
                .arg(
                    Arg::new("USERGROUP")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .help("Group name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-usergroup-from-usergroup")
                .about("Delete user group from user group")
                .override_help("oerec-delete-usergroup-from-usergroup
Delete user group from user group

USAGE:
    oerec delete-usergroup-from-usergroup [OPTIONS]

OPTIONS:
        --subgroup <SUBGROUP>        Member user group name
        --supergroup <SUPERGROUP>    Parent user group name
        --confirm                    Skip confirmation dialog

    -h, --help                       Print this message")
                .display_order(300)
                .arg(
                    Arg::new("SUBGROUP")
                        .long("subgroup")
                        .help("Member user group name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SUPERGROUP")
                        .long("supergroup")
                        .help("Parent user group name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-usergroup")
                .about("Delete user group")
                .override_help("oerec-delete-usergroup
Delete user group

USAGE:
    oerec delete-usergroup [OPTIONS]

OPTIONS:
        --usergroup <USERGROUP>    [alias: --groupname]
        --confirm                  Skip confirmation dialog

    -h, --help                     Print this message")
                .display_order(300)
                .arg(
                    Arg::new("USERGROUP")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-servergroup")
                .about("Delete server group")
                .override_help("oerec-delete-servergroup
Delete server group

USAGE:
    oerec delete-servergroup [OPTIONS]

OPTIONS:
        --servergroup <SERVERGROUP>    [alias: --groupname]
        --confirm                      Skip confirmation dialog

    -h, --help                         Print this message")
                .display_order(300)
                .arg(
                    Arg::new("SERVERGROUP")
                        .long("servergroup")
                        .visible_alias("groupname")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-server-from-servergroup")
                .about("Delete server from server group")
                .override_help("oerec-delete-server-from-servergroup
Delete server from server group

USAGE:
    oerec delete-server-from-servergroup [OPTIONS]

OPTIONS:
        --server <SERVER>              Server Name
        --servergroup <SERVERGROUP>    Group name [alias: --groupname]
        --confirm                      Skip confirmation dialog

    -h, --help                         Print this message")
                .display_order(300)
                .arg(
                    Arg::new("SERVER")
                        .long("server")
                        .help("Server Name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERGROUP")
                        .long("servergroup")
                        .visible_alias("groupname")
                        .help("Group name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-servergroup-from-servergroup")
                .about("Delete server group from server group")
                .override_help("oerec-delete-servergroup-from-servergroup
Delete server group from server group

USAGE:
    oerec delete-servergroup-from-servergroup [OPTIONS]

OPTIONS:
        --subgroup <SUBGROUP>        Member server group name
        --supergroup <SUPERGROUP>    Parent server group name
        --confirm                    Skip confirmation dialog

    -h, --help                       Print this message")
                .display_order(300)
                .arg(
                    Arg::new("SUBGROUP")
                        .long("subgroup")
                        .help("Member server group name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SUPERGROUP")
                        .long("supergroup")
                        .help("Parent server group name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-serveraccess")
                .about("Delete server access")
                .override_help("oerec-delete-serveraccess
Delete server access

USAGE:
    oerec delete-serveraccess [OPTIONS]

OPTIONS:
        --serveraccess <SERVERACCESS>    [alias: --name]
        --confirm                        Skip confirmation dialog

    -h, --help                           Print this message")
                .display_order(300)
                .arg(
                    Arg::new("SERVERACCESS")
                        .long("serveraccess")
                        .visible_alias("name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-useraccess")
                .about("Delete user access")
                .override_help("oerec-delete-useraccess
Delete user access

USAGE:
    oerec delete-useraccess [OPTIONS]

OPTIONS:
        --email <EMAIL>
        --usergroup <USERGROUP>          [alias: --groupname]
        --serveraccess <SERVERACCESS>    [alias: --name]
        --confirm                        Skip confirmation dialog

    -h, --help                           Print this message")
                .display_order(300)
                .arg(Arg::new("EMAIL").long("email").takes_value(true))
                .arg(
                    Arg::new("USERGROUP")
                        .conflicts_with("EMAIL")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERACCESS")
                        .long("serveraccess")
                        .visible_alias("name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("disable-user")
                .about("Disable user")
                .override_help("oerec-disable-user
Disable user

USAGE:
    oerec disable-user [OPTIONS]

OPTIONS:
        --email <EMAIL>
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(400)
                .arg(
                    Arg::new("USEREMAIL")
                        .long("email")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("enable-user")
                .about("Enable user")
                .override_help("oerec-enable-user
Enable user

USAGE:
    oerec enable-user [OPTIONS]

OPTIONS:
        --email <EMAIL>
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(400)
                .arg(
                    Arg::new("USEREMAIL")
                        .long("email")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("disable-server")
                .about("Disable server")
                .override_help("oerec-disable-server
Disable server

USAGE:
    oerec disable-server [OPTIONS]

OPTIONS:
        --server <NAME>    [alias: --name]
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(400)
                .arg(
                    Arg::new("NAME")
                        .long("server")
                        .visible_alias("name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("enable-server")
                .about("Enable server")
                .override_help("oerec-enable-server
Enable server

USAGE:
    oerec enable-server [OPTIONS]

OPTIONS:
        --server <NAME>    [aliases: name]
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(400)
                .arg(
                    Arg::new("NAME")
                        .long("server")
                        .visible_alias("name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("disable-dns")
                .about("Disable server DNS lookup")
                .override_help("oerec-disable-dns
Disable server DNS lookup

USAGE:
    oerec disable-dns [OPTIONS]

OPTIONS:
        --server <NAME>    [alias: --name]
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(400)
                .arg(
                    Arg::new("NAME")
                        .long("server")
                        .visible_alias("name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("enable-dns")
                .about("Enable server DNS lookup")
                .override_help("oerec-enable-dns
Enable server DNS lookup

USAGE:
    oerec enable-dns [OPTIONS]

OPTIONS:
        --server <NAME>    [alias: --name]
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(400)
                .arg(
                    Arg::new("NAME")
                        .long("server")
                        .visible_alias("name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .help("Skip confirmation dialog")
                ),
            Command::new("update-server")
                .about("Update server")
                .override_help("oerec-update-server
Update server

USAGE:
    oerec update-server [OPTIONS]

OPTIONS:
        --id <SERVERID>        [alias: --serverid]
        --server <NAME>        Server name [alias: --name]
        --ip <IP>
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(500)
                .arg(
                    Arg::new("SERVERID")
                        .long("id")
                        .visible_alias("serverid")
                        .takes_value(true)
                )
                .arg(
                    Arg::new("NAME")
                        .long("server")
                        .visible_alias("name")
                        .help("Server name")
                        .takes_value(true),
                )
                .arg(Arg::new("IP").long("ip").takes_value(true))
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("update-servergroup")
                .about("Update server group")
                .override_help("oerec-update-servergroup
Update server group

USAGE:
    oerec update-servergroup [OPTIONS]

OPTIONS:
        --servergroup <SERVERGROUP>     [alias: --groupname]
        --newname <NEWNAME>             New group name
        --comment <COMMENT>

    -h, --help                          Print this message")
                .display_order(500)
                .arg(
                    Arg::new("SERVERGROUP")
                        .long("servergroup")
                        .visible_alias("groupname")
                        .takes_value(true)
                )
                .arg(
                    Arg::new("NEWNAME")
                        .long("newname")
                        .help("New group name")
                        .takes_value(true),
                )
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("update-user")
                .about("Update user")
                .override_help("oerec-update-user
Update user

USAGE:
    oerec update-user [OPTIONS]

OPTIONS:
        --id <USERID>          [alias: --userid]
        --email <EMAIL>
        --name <NAME>
        --type <TYPE>          `AD user` / `tool user` / `external user`
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(500)
                .arg(Arg::new("USERID").long("id").visible_alias("userid").takes_value(true))
                .arg(Arg::new("EMAIL").long("email").takes_value(true))
                .arg(Arg::new("NAME").long("name").takes_value(true))
                .arg(Arg::new("TYPE").long("type").takes_value(true))
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("update-usergroup")
                .about("Update user group")
                .override_help("oerec-update-usergroup
Update user group

USAGE:
    oerec update-usergroup [OPTIONS]

OPTIONS:
        --usergroup <USERGROUP>    [alias: --groupname]
        --newname <NEWNAME>        New group name
        --comment <COMMENT>

    -h, --help                     Print this message")
                .display_order(500)
                .arg(
                    Arg::new("USERGROUP")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .takes_value(true)
                )
                .arg(
                    Arg::new("NEWNAME")
                        .long("newname")
                        .help("New group name")
                        .takes_value(true),
                )
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("update-key")
                .alias("update-sshkey")
                .about("Update public SSH key")
                .override_help("oerec-update-key
Update public SSH key

USAGE:
    oerec update-key [OPTIONS]

OPTIONS:
        --id <KEYID>           [alias: --keyid]
        --sshkey <KEY>
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(500)
                .arg(Arg::new("KEYID").long("id").visible_alias("keyid").takes_value(true))
                .arg(Arg::new("KEY").long("sshkey").takes_value(true))
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("update-serveraccess")
                .about("Update server access")
                .override_help("oerec-update-serveraccess
Update server access

USAGE:
    oerec update-serveraccess [OPTIONS]

OPTIONS:
        --serveraccess <NAME>          Server access name [alias: --name]
        --newname <NEWNAME>            New server access name [alias: --newserveraccess]
        --user <SSHUSER>               SSH / OS user [alias: --sshuser]

        --sshfrom <SSHFROM>            from= pattern-list
        --sshcommand <SSHCOMMAND>      command= pattern
        --sshoption <SSHOPTION>        additional options, e.g `no-pty`

        --server <SERVER>              Server name
        --servergroup <SERVERGROUP>    Server group name
        --comment <COMMENT>

    -h, --help                         Print this message")
                .display_order(500)
                .arg(
                    Arg::new("NAME")
                        .long("serveraccess")
                        .visible_alias("name")
                        .help("Server access name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("NEWNAME")
                        .long("newname")
                        .visible_alias("newserveraccess")
                        .help("New server access name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SSHUSER")
                        .long("user")
                        .visible_alias("sshuser")
                        .help("SSH / OS user")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SSHFROM")
                        .long("sshfrom")
                        .help("from= pattern-list")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SSHCOMMAND")
                        .long("sshcommand")
                        .help("command= pattern")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SSHOPTION")
                        .long("sshoption")
                        .help("additional options, e.g `no-pty`")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVER")
                        .long("server")
                        .help("Server name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("SERVERGROUP")
                        .conflicts_with("SERVER")
                        .long("servergroup")
                        .help("Server group name")
                        .takes_value(true),
                )
                .arg(Arg::new("COMMENT").long("comment").takes_value(true)),
            Command::new("write-serverauth")
                .about("Write authorized_keys to workdir")
                .override_help("oerec-write-serverauth
Write authorized_keys to workdir

USAGE:
    oerec write-serverauth [OPTIONS] --workdir <WORKDIR>

OPTIONS:
        --workdir <WORKDIR>    [alias: --dir]
        --force                Overwrite workdir contents (USE WITH CAUTION)

    -h, --help                 Print this message")
                .display_order(600)
                .arg(
                    Arg::new("WORKDIR")
                        .long("workdir")
                        .visible_alias("dir")
                        .required(true)
                        .value_parser(ValueParser::os_string())
                        .takes_value(true),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("force")
                        .help("Overwrite workdir contents (USE WITH CAUTION)")
                ),
        ])
}
