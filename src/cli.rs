use clap::builder::ValueParser;
use clap::{Arg, ArgAction, Command};

#[allow(clippy::too_many_lines)]
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::must_use_candidate)]
pub fn build_cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .disable_colored_help(true)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("\u{d8}re client")
        .long_about("Does awesome things")
        .override_help("oerec
\u{d8}re client

Usage: oerec [OPTIONS] [COMMAND]

Options:
    -h, --help       Print this message or the help of the given subcommand
    -j, --json       Set output mode to JSON for commands that support it (list-*)
    -V, --version    Print version information

Commands:
    add-server, list-server, update-server, delete-server

    add-servergroup, list-servergroup, update-servergroup, delete-servergroup

    add-server-to-servergroup, add-servergroup-to-servergroup
    delete-server-from-servergroup, delete-servergroup-from-servergroup

    add-user, list-user, update-user, delete-user

    add-key, list-key, update-key, delete-key

    add-usergroup, list-usergroup, update-usergroup, delete-usergroup

    add-user-to-usergroup, add-usergroup-to-usergroup
    delete-user-from-usergroup, delete-usergroup-from-usergroup

    add-serveraccess, list-serveraccess, update-serveraccess, delete-serveraccess
    add-useraccess, list-useraccess, delete-useraccess

    enable-dns, disable-dns
    enable-server, disable-server
    enable-user, disable-user

    write-serverauth

    help")
        .arg(
            Arg::new("JSONOUTPUT")
                .long("json")
                .short('j')
                .action(ArgAction::SetTrue)
                .help("Return list-* output as JSON object"),
        ).
        subcommands(vec![
            Command::new("list-user")
                .alias("list-users")
                .about("List user accounts")
                .override_help("oerec-list-user
List user accounts

Usage: oerec list-user [OPTIONS]

Options:
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
                        .num_args(1),
                )
                .arg(
                    Arg::new("ID")
                        .long("id")
                        .help("List user w/ ID")
                        .num_args(1),
                )
                .arg(
                    Arg::new("NAME")
                        .long("name")
                        .help("List user by name [LIKE %NAME%]")
                        .num_args(1),
                )
                .arg(
                    Arg::new("EXACT")
                        .long("exact")
                        .short('e')
                        .action(ArgAction::SetTrue)
                        .help("Only list exact matches")
                ),
            Command::new("list-key")
                .alias("list-keys")
                .about("List SSH keys")
                .override_help("oerec-list-key
List SSH keys

Usage: oerec list-key [OPTIONS]

Options:
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
                        .num_args(1),
                )
                .arg(
                    Arg::new("ID")
                        .long("id")
                        .help("List key w/ ID")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FINGERPRINT")
                        .long("fingerprint")
                        .help("List keys by SHA256 fingerprint")
                        .num_args(1),
                )
                .arg(
                    Arg::new("WITHKEY")
                        .long("with-key")
                        .action(ArgAction::SetTrue)
                        .visible_alias("long")
                        .help("List public SSH keys"),
                ),
            Command::new("list-server")
                .about("List managed servers")
                .override_help("oerec-list-server
List managed servers

Usage: oerec list-server [OPTIONS]

Options:
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
                        .num_args(1),
                )
                .arg(
                    Arg::new("ID")
                        .long("id")
                        .help("List server w/ ID")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERNAME")
                        .long("server")
                        .visible_alias("name")
                        .help("List server by name [LIKE %SERVERNAME%]")
                        .num_args(1),
                )
                .arg(
                    Arg::new("EXACT")
                        .long("exact")
                        .short('e')
                        .action(ArgAction::SetTrue)
                        .help("Only list exact matches")
                ),
            Command::new("list-serverauth")
                .hide(true)
                .about("List server auth")
                .override_help("oerec-list-serverauth
List server auth

Usage: oerec list-serverauth [OPTIONS]

Options:
        --ip <IP>                List server auth by IP
        --server <SERVERNAME>    List server auth by SERVERNAME

    -h, --help                   Print this message")
                .display_order(200)
                .arg(
                    Arg::new("IP")
                        .long("ip")
                        .help("List server auth by IP")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERNAME")
                        .long("server")
                        .help("List server auth by name")
                        .conflicts_with("IP")
                        .num_args(1),
                ),
            Command::new("list-servergroup")
                .alias("list-servergroups")
                .about("List server groups")
                .override_help("oerec-list-servergroup
List server groups

Usage: oerec list-servergroup [OPTIONS]

Options:
        --ip <IP>                      List server group containing server w/ IP
        --server <SERVERNAME>          List server group containing server w/ SERVERNAME

        --all                          include the 'all' server group
        --empty                        List only server groups w/o members
    -e, --exact                        Only list exact matches

    -h, --help                         Print this message

Filter:
        --servergroup <SERVERGROUP>    Filter output by group name [alias: --groupname]")
                .display_order(200)
                .arg(
                    Arg::new("SERVERGROUP")
                        .long("servergroup")
                        .visible_alias("groupname")
                        .help("WARNING: This is just a filter.\nFilter server group by name\nThis will *not* generate a complete list.")
                        .num_args(1),
                )
                .arg(
                    Arg::new("IP")
                        .long("ip")
                        .help("List server group containing server w/ IP [LIKE %IP%]")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERNAME")
                        .long("server")
                        .help("List server group containing server w/ SERVERNAME [LIKE %SERVERNAME%]")
                        .num_args(1),
                )
                .arg(
                    Arg::new("ALL")
                        .long("all")
                        .action(ArgAction::SetTrue)
                        .help("include the 'all' server group"),
                )
                .arg(
                    Arg::new("EMPTY")
                        .long("empty")
                        .action(ArgAction::SetTrue)
                        .help("List only server groups w/o members"),
                )
                .arg(
                    Arg::new("EXACT")
                        .long("exact")
                        .short('e')
                        .action(ArgAction::SetTrue)
                        .help("Only list exact matches")
                ),
            Command::new("list-usergroup")
                .alias("list-usergroups")
                .about("List user groups")
                .override_help("oerec-list-usergroup
List user groups

Usage: oerec list-usergroup [OPTIONS]

Options:
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
                        .num_args(1),
                )
                .arg(
                    Arg::new("NAME")
                        .long("groupname")
                        .visible_alias("usergroup")
                        .help("List user group w/ name [LIKE %NAME%]")
                        .num_args(1),
                )
                .arg(
                    Arg::new("EMPTY")
                        .long("empty")
                        .action(ArgAction::SetTrue)
                        .help("List only user groups w/o members"),
                )
                .arg(
                    Arg::new("EXACT")
                        .long("exact")
                        .short('e')
                        .action(ArgAction::SetTrue)
                        .help("Only list exact matches")
                ),
            Command::new("list-useraccess")
                .about("List user access")
                .override_help("oerec-list-useraccess
List user access

Usage: oerec list-useraccess [OPTIONS]

Options:
        --ip <IP>                       List user access on server w/ IP
        --server <SERVERNAME>           List user access on server SERVERNAME
        --email <EMAIL>                 List useraccess containing member w/ EMAIL
        --sshuser <USER>                List user access w/ SSH USER [aliases: --user, --osuser]
        --serveraccess <SERVERACCESS>   List user / user group w/ access to SERVERACCESS

        --expired                       List only expired useraccess entries
    -e, --exact                         Only list exact matches

    -h, --help                          Print this message

Filter:
        --servergroup <SERVERGROUP>     Filter output by server group
        --usergroup <USERGROUP>         Filter output by user group")
                .display_order(200)
                .arg(
                    Arg::new("EMAIL")
                        .long("email")
                        .help("List useraccess containing member w/ EMAIL")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERNAME")
                        .long("server")
                        .help("List user access on server SERVERNAME")
                        .num_args(1),
                )
                .arg(
                    Arg::new("IP")
                        .long("ip")
                        .help("List user access on server w/ IP")
                        .num_args(1),
                )
                .arg(
                    Arg::new("USER")
                        .long("sshuser")
                        .visible_alias("user")
                        .alias("osuser")
                        .help("List user access w/ SSH USER")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERACCESS")
                        .long("serveraccess")
                        .help("List user access w/ SERVERACCESS")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERGROUP")
                        .long("servergroup")
                        .help("WARNING: This is just a filter.\nFilter user access on server group\nThis will *not* generate a complete list.")
                        .num_args(1),
                )
                .arg(
                    Arg::new("USERGROUP")
                        .long("usergroup")
                        .help("WARNING: This is just a filter.\nFilter server access by user group\nThis will *not* generate a complete list.")
                        .num_args(1),
                )
                .arg(
                    Arg::new("EXPIRED")
                        .long("expired")
                        .action(ArgAction::SetTrue)
                        .help("List only expired useraccess entries"),
                )
                .arg(
                Arg::new("EXACT")
                    .long("exact")
                    .short('e')
                    .action(ArgAction::SetTrue)
                    .help("Only list exact matches")
                ),
            Command::new("list-serveraccess")
                .about("List server access")
                .override_help("oerec-list-serveraccess
List server access

Usage: oerec list-serveraccess [OPTIONS]

Options:
        --ip <IP>                      List server access on server w/ IP
        --server <SERVERNAME>          List server access on server SERVERNAME
        --user <SSHUSER>               List server access w/ SSHUSER user [alias: --sshuser]

    -e, --exact                        Only list exact matches

    -h, --help                         Print this message

Filter:
        --serveraccess <NAME>          Filter output by NAME [alias: --name]
        --servergroup <SERVERGROUP>    Filter output by SERVERGROUP")
                .display_order(200)
                .arg(
                    Arg::new("NAME")
                        .long("serveraccess")
                        .visible_alias("name")
                        .help("WARNING: This is just a filter.\nFilter server access by name\nThis will *not* generate a complete list.")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERNAME")
                        .long("server")
                        .help("List server access on server SERVERNAME [LIKE %SERVERNAME%]")
                        .num_args(1),
                )
                .arg(
                    Arg::new("IP")
                        .long("ip")
                        .help("List server access on server w/ IP [LIKE %IP%]")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SSHUSER")
                        .long("user")
                        .visible_alias("sshuser")
                        .help("List server access w/ SSHUSER user [LIKE %SSHUSER")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERGROUP")
                        .long("servergroup")
                        .help("WARNING: This is just a filter.\nFilter server access by server group\nThis will *not* generate a complete list.")
                        .num_args(1),
                )
                .arg(
                    Arg::new("EXACT")
                        .long("exact")
                        .short('e')
                        .action(ArgAction::SetTrue)
                        .help("Only list exact matches")
                ),
            Command::new("add-server")
                .about("Add server")
                .override_help("oerec-add-server
Add server

Usage: oerec add-server [OPTIONS]

Options:
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
                        .num_args(1),
                )
                .arg(Arg::new("IP")
                    .long("ip")
                    .num_args(1))
                .arg(Arg::new("DISABLED")
                    .long("disabled")
                    .action(ArgAction::SetTrue)
                    .help("Add server but disable key distribution"))
                .arg(Arg::new("DNS")
                    .long("enable-dns")
                    .action(ArgAction::SetTrue)
                    .alias("usedns")
                    .help("Resolve server name to determine target IP"))
                .arg(Arg::new("COMMENT")
                    .long("comment")
                    .num_args(1)),
            Command::new("add-user")
                .about("Add user")
                .override_help("oerec-add-user
Add user

Usage: oerec add-user [OPTIONS]

Options:
        --email <EMAIL>
        --name <NAME>
        --type <TYPE>          `AD user` / `tool user` / `external user`
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(100)
                .arg(Arg::new("EMAIL").long("email").num_args(1))
                .arg(Arg::new("NAME").long("name").num_args(1))
                .arg(Arg::new("TYPE").long("type").num_args(1))
                .arg(Arg::new("COMMENT").long("comment").num_args(1)),
            Command::new("add-key")
                .alias("add-sshkey")
                .about("Add public SSH key")
                .override_help("oerec-add-key
Add public SSH key

Usage: oerec add-key [OPTIONS]

Options:
        --email <EMAIL>
        --sshkey <KEY>
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(100)
                .arg(Arg::new("EMAIL").long("email").num_args(1))
                .arg(Arg::new("KEY").long("sshkey").num_args(1))
                .arg(Arg::new("COMMENT").long("comment").num_args(1)),
            Command::new("add-usergroup")
                .about("Add user group")
                .override_help("oerec-add-usergroup
Add user group

Usage: oerec add-usergroup [OPTIONS]

Options:
        --usergroup <NAME>     Group name [alias: --groupname]
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(100)
                .arg(
                    Arg::new("NAME")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .help("Group name")
                        .num_args(1),
                )
                .arg(Arg::new("COMMENT").long("comment").num_args(1)),
            Command::new("add-user-to-usergroup")
                .about("Add user to user group")
                .override_help("oerec-add-user-to-usergroup
Add user to user group

Usage: oerec add-user-to-usergroup [OPTIONS]

Options:
        --usergroup <NAME>    Group name [alias: --groupname]
        --email <EMAIL>

    -h, --help                Print this message")
                .display_order(100)
                .arg(
                    Arg::new("NAME")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .help("Group name")
                        .num_args(1),
                )
                .arg(Arg::new("EMAIL").long("email").num_args(1)),
            Command::new("add-usergroup-to-usergroup")
                .about("Add user group to user group")
                .override_help("oerec-add-usergroup-to-usergroup
Add user group to user group

Usage: oerec add-usergroup-to-usergroup [OPTIONS]

Options:
        --subgroup <SUBGROUP>        Member user group name
        --supergroup <SUPERGROUP>    Parent user group name

    -h, --help                       Print this message")
                .display_order(100)
                .arg(
                    Arg::new("SUBGROUP")
                        .long("subgroup")
                        .help("Member user group name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SUPERGROUP")
                        .long("supergroup")
                        .help("Parent user group name")
                        .num_args(1),
                ),
            Command::new("add-servergroup")
                .about("Add server group")
                .override_help("oerec-add-servergroup
Add server group

Usage: oerec add-servergroup [OPTIONS]

Options:
        --servergroup <NAME>     Group name [alias: --groupname]
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(100)
                .arg(
                    Arg::new("NAME")
                        .long("servergroup")
                        .visible_alias("groupname")
                        .help("Group name")
                        .num_args(1),
                )
                .arg(Arg::new("COMMENT").long("comment").num_args(1)),
            Command::new("add-server-to-servergroup")
                .about("Add server to server group")
                .override_help("oerec-add-server-to-servergroup
Add server to server group

Usage: oerec add-server-to-servergroup [OPTIONS]

Options:
        --servergroup <NAME>    Group name [alias: --groupname]
        --server <SERVER>       Server name

    -h, --help                  Print this message")
                .display_order(100)
                .arg(
                    Arg::new("NAME")
                        .long("servergroup")
                        .visible_alias("groupname")
                        .help("Group name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVER")
                        .long("server")
                        .help("Server name")
                        .num_args(1),
                ),
            Command::new("add-servergroup-to-servergroup")
                .about("Add server group to server group")
                .override_help("oerec-add-servergroup-to-servergroup
Add server group to server group

Usage: oerec add-servergroup-to-servergroup [OPTIONS]

Options:
        --subgroup <SUBGROUP>        Member server group name
        --supergroup <SUPERGROUP>    Parent server group name

    -h, --help                       Print this message")
                .display_order(100)
                .arg(
                    Arg::new("SUBGROUP")
                        .long("subgroup")
                        .help("Member server group name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SUPERGROUP")
                        .long("supergroup")
                        .help("Parent server group name")
                        .num_args(1),
                ),
            Command::new("add-serveraccess")
                .about("Add server access\n\nAdd access to server or server group.\nYou'll have to specify either --server *or* --servergroup.")
                .override_help("oerec-add-serveraccess
Add server access

Add access to server or server group.
You'll have to specify either --server *or* --servergroup.

Usage: oerec add-serveraccess [OPTIONS] [--server <SERVER> | --servergroup <SERVERGROUP>]

Options:
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
                        .num_args(1),
                )
                .arg(
                    Arg::new("SSHUSER")
                        .long("user")
                        .visible_alias("sshuser")
                        .help("SSH / OS user")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SSHFROM")
                        .long("sshfrom")
                        .help("from= pattern-list")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SSHCOMMAND")
                        .long("sshcommand")
                        .help("command= pattern")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SSHOPTION")
                        .long("sshoption")
                        .help("Additional key options (`man 8 sshd`)")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVER")
                        .long("server")
                        .help("Server name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERGROUP")
                        .conflicts_with("SERVER")
                        .long("servergroup")
                        .help("Server group name")
                        .num_args(1),
                )
                .arg(Arg::new("COMMENT").long("comment").num_args(1)),
            Command::new("add-useraccess")
                .about("Add user access\n\nAdd either user (via email) *or* usergroup (via user group name) to server access.")
                .override_help("oerec-add-useraccess
Add user access

Add either user (via email) *or* usergroup (via user group name) to server access.

Usage: oerec add-useraccess [OPTIONS] [ --email <EMAIL> | --usergroup <USERGROUP> ]

Options:
        --email <EMAIL>
        --usergroup <USERGROUP>          [alias: --groupname]
        --serveraccess <SERVERACCESS>
        --until <UNTIL>                  Format: YYYY-MM-DD, optional w/ HH:MI:SS
        --comment <COMMENT>

    -h, --help                           Print this message")
                .display_order(100)
                .arg(Arg::new("EMAIL").long("email").num_args(1))
                .arg(
                    Arg::new("USERGROUP")
                        .conflicts_with("EMAIL")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERACCESS")
                        .long("serveraccess")
                        .num_args(1),
                )
                .arg(Arg::new("UNTIL").long("until").help("Format: YYYY-MM-DD, optional w/ HH:MI:SS").num_args(1))
                .arg(Arg::new("COMMENT").long("comment").num_args(1)),
            Command::new("delete-server")
                .about("Delete server")
                .override_help("oerec-delete-server
Delete server

Usage: oerec delete-server [OPTIONS]

Options:
        --server <SERVER>    [alias: --servername]
        --confirm            Skip confirmation dialog

    -h, --help               Print this message")
                .display_order(300)
                .arg(
                    Arg::new("SERVER")
                        .long("server")
                        .alias("name")
                        .visible_alias("servername")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-user")
                .about("Delete user")
                .override_help("oerec-delete-user
Delete user

Usage: oerec delete-user [OPTIONS]

Options:
        --email <EMAIL>
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(300)
                .arg(
                    Arg::new("EMAIL")
                        .long("email")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-key")
                .about("Delete SSH key")
                .override_help("oerec-delete-key
Delete SSH key

Usage: oerec delete-key [OPTIONS]

Options:
        --id <KEYID>
        --confirm       Skip confirmation dialog

    -h, --help          Print this message")
                .display_order(300)
                .arg(
                    Arg::new("KEYID")
                        .long("id")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-user-from-usergroup")
                .about("Delete user from user group")
                .override_help("oerec-delete-user-from-usergroup
Delete user from user group

Usage: oerec delete-user-from-usergroup [OPTIONS]

Options:
        --email <EMAIL>
        --usergroup <USERGROUP>    Group name [alias: --groupname]
        --confirm                  Skip confirmation dialog

    -h, --help                     Print this message")
                .display_order(300)
                .arg(Arg::new("EMAIL").long("email").num_args(1))
                .arg(
                    Arg::new("USERGROUP")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .help("Group name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-usergroup-from-usergroup")
                .about("Delete user group from user group")
                .override_help("oerec-delete-usergroup-from-usergroup
Delete user group from user group

Usage: oerec delete-usergroup-from-usergroup [OPTIONS]

Options:
        --subgroup <SUBGROUP>        Member user group name
        --supergroup <SUPERGROUP>    Parent user group name
        --confirm                    Skip confirmation dialog

    -h, --help                       Print this message")
                .display_order(300)
                .arg(
                    Arg::new("SUBGROUP")
                        .long("subgroup")
                        .help("Member user group name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SUPERGROUP")
                        .long("supergroup")
                        .help("Parent user group name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-usergroup")
                .about("Delete user group")
                .override_help("oerec-delete-usergroup
Delete user group

Usage: oerec delete-usergroup [OPTIONS]

Options:
        --usergroup <USERGROUP>    [alias: --groupname]
        --confirm                  Skip confirmation dialog

    -h, --help                     Print this message")
                .display_order(300)
                .arg(
                    Arg::new("USERGROUP")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-servergroup")
                .about("Delete server group")
                .override_help("oerec-delete-servergroup
Delete server group

Usage: oerec delete-servergroup [OPTIONS]

Options:
        --servergroup <SERVERGROUP>    [alias: --groupname]
        --confirm                      Skip confirmation dialog

    -h, --help                         Print this message")
                .display_order(300)
                .arg(
                    Arg::new("SERVERGROUP")
                        .long("servergroup")
                        .visible_alias("groupname")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-server-from-servergroup")
                .about("Delete server from server group")
                .override_help("oerec-delete-server-from-servergroup
Delete server from server group

Usage: oerec delete-server-from-servergroup [OPTIONS]

Options:
        --server <SERVER>              Server Name
        --servergroup <SERVERGROUP>    Group name [alias: --groupname]
        --confirm                      Skip confirmation dialog

    -h, --help                         Print this message")
                .display_order(300)
                .arg(
                    Arg::new("SERVER")
                        .long("server")
                        .help("Server Name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERGROUP")
                        .long("servergroup")
                        .visible_alias("groupname")
                        .help("Group name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-servergroup-from-servergroup")
                .about("Delete server group from server group")
                .override_help("oerec-delete-servergroup-from-servergroup
Delete server group from server group

Usage: oerec delete-servergroup-from-servergroup [OPTIONS]

Options:
        --subgroup <SUBGROUP>        Member server group name
        --supergroup <SUPERGROUP>    Parent server group name
        --confirm                    Skip confirmation dialog

    -h, --help                       Print this message")
                .display_order(300)
                .arg(
                    Arg::new("SUBGROUP")
                        .long("subgroup")
                        .help("Member server group name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SUPERGROUP")
                        .long("supergroup")
                        .help("Parent server group name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-serveraccess")
                .about("Delete server access")
                .override_help("oerec-delete-serveraccess
Delete server access

Usage: oerec delete-serveraccess [OPTIONS]

Options:
        --serveraccess <SERVERACCESS>    [alias: --name]
        --confirm                        Skip confirmation dialog

    -h, --help                           Print this message")
                .display_order(300)
                .arg(
                    Arg::new("SERVERACCESS")
                        .long("serveraccess")
                        .visible_alias("name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("delete-useraccess")
                .about("Delete user access")
                .override_help("oerec-delete-useraccess
Delete user access

Usage: oerec delete-useraccess [OPTIONS]

Options:
        --email <EMAIL>
        --usergroup <USERGROUP>          [alias: --groupname]
        --serveraccess <SERVERACCESS>    [alias: --name]
        --confirm                        Skip confirmation dialog

    -h, --help                           Print this message")
                .display_order(300)
                .arg(Arg::new("EMAIL").long("email").num_args(1))
                .arg(
                    Arg::new("USERGROUP")
                        .conflicts_with("EMAIL")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERACCESS")
                        .long("serveraccess")
                        .visible_alias("name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("disable-user")
                .about("Disable user")
                .override_help("oerec-disable-user
Disable user

Usage: oerec disable-user [OPTIONS]

Options:
        --email <EMAIL>
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(400)
                .arg(
                    Arg::new("USEREMAIL")
                        .long("email")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("enable-user")
                .about("Enable user")
                .override_help("oerec-enable-user
Enable user

Usage: oerec enable-user [OPTIONS]

Options:
        --email <EMAIL>
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(400)
                .arg(
                    Arg::new("USEREMAIL")
                        .long("email")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("disable-server")
                .about("Disable server")
                .override_help("oerec-disable-server
Disable server

Usage: oerec disable-server [OPTIONS]

Options:
        --server <NAME>    [alias: --name]
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(400)
                .arg(
                    Arg::new("NAME")
                        .long("server")
                        .visible_alias("name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("enable-server")
                .about("Enable server")
                .override_help("oerec-enable-server
Enable server

Usage: oerec enable-server [OPTIONS]

Options:
        --server <NAME>    [aliases: name]
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(400)
                .arg(
                    Arg::new("NAME")
                        .long("server")
                        .visible_alias("name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("disable-dns")
                .about("Disable server DNS lookup")
                .override_help("oerec-disable-dns
Disable server DNS lookup

Usage: oerec disable-dns [OPTIONS]

Options:
        --server <NAME>    [alias: --name]
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(400)
                .arg(
                    Arg::new("NAME")
                        .long("server")
                        .visible_alias("name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("enable-dns")
                .about("Enable server DNS lookup")
                .override_help("oerec-enable-dns
Enable server DNS lookup

Usage: oerec enable-dns [OPTIONS]

Options:
        --server <NAME>    [alias: --name]
        --confirm          Skip confirmation dialog

    -h, --help             Print this message")
                .display_order(400)
                .arg(
                    Arg::new("NAME")
                        .long("server")
                        .visible_alias("name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("confirm")
                        .action(ArgAction::SetTrue)
                        .help("Skip confirmation dialog")
                ),
            Command::new("update-server")
                .about("Update server")
                .override_help("oerec-update-server
Update server

Usage: oerec update-server [OPTIONS]

Options:
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
                        .num_args(1)
                )
                .arg(
                    Arg::new("NAME")
                        .long("server")
                        .visible_alias("name")
                        .help("Server name")
                        .num_args(1),
                )
                .arg(Arg::new("IP").long("ip").num_args(1))
                .arg(Arg::new("COMMENT").long("comment").num_args(1)),
            Command::new("update-servergroup")
                .about("Update server group")
                .override_help("oerec-update-servergroup
Update server group

Usage: oerec update-servergroup [OPTIONS]

Options:
        --servergroup <SERVERGROUP>     [alias: --groupname]
        --newname <NEWNAME>             New group name
        --comment <COMMENT>

    -h, --help                          Print this message")
                .display_order(500)
                .arg(
                    Arg::new("SERVERGROUP")
                        .long("servergroup")
                        .visible_alias("groupname")
                        .num_args(1)
                )
                .arg(
                    Arg::new("NEWNAME")
                        .long("newname")
                        .help("New group name")
                        .num_args(1),
                )
                .arg(Arg::new("COMMENT").long("comment").num_args(1)),
            Command::new("update-user")
                .about("Update user")
                .override_help("oerec-update-user
Update user

Usage: oerec update-user [OPTIONS]

Options:
        --id <USERID>          [alias: --userid]
        --email <EMAIL>
        --name <NAME>
        --type <TYPE>          `AD user` / `tool user` / `external user`
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(500)
                .arg(Arg::new("USERID").long("id").visible_alias("userid").num_args(1))
                .arg(Arg::new("EMAIL").long("email").num_args(1))
                .arg(Arg::new("NAME").long("name").num_args(1))
                .arg(Arg::new("TYPE").long("type").num_args(1))
                .arg(Arg::new("COMMENT").long("comment").num_args(1)),
            Command::new("update-usergroup")
                .about("Update user group")
                .override_help("oerec-update-usergroup
Update user group

Usage: oerec update-usergroup [OPTIONS]

Options:
        --usergroup <USERGROUP>    [alias: --groupname]
        --newname <NEWNAME>        New group name
        --comment <COMMENT>

    -h, --help                     Print this message")
                .display_order(500)
                .arg(
                    Arg::new("USERGROUP")
                        .long("usergroup")
                        .visible_alias("groupname")
                        .num_args(1)
                )
                .arg(
                    Arg::new("NEWNAME")
                        .long("newname")
                        .help("New group name")
                        .num_args(1),
                )
                .arg(Arg::new("COMMENT").long("comment").num_args(1)),
            Command::new("update-key")
                .alias("update-sshkey")
                .about("Update public SSH key")
                .override_help("oerec-update-key
Update public SSH key

Usage: oerec update-key [OPTIONS]

Options:
        --id <KEYID>           [alias: --keyid]
        --sshkey <KEY>
        --comment <COMMENT>

    -h, --help                 Print this message")
                .display_order(500)
                .arg(Arg::new("KEYID").long("id").visible_alias("keyid").num_args(1))
                .arg(Arg::new("KEY").long("sshkey").num_args(1))
                .arg(Arg::new("COMMENT").long("comment").num_args(1)),
            Command::new("update-serveraccess")
                .about("Update server access")
                .override_help("oerec-update-serveraccess
Update server access

Usage: oerec update-serveraccess [OPTIONS]

Options:
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
                        .num_args(1),
                )
                .arg(
                    Arg::new("NEWNAME")
                        .long("newname")
                        .visible_alias("newserveraccess")
                        .help("New server access name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SSHUSER")
                        .long("user")
                        .visible_alias("sshuser")
                        .help("SSH / OS user")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SSHFROM")
                        .long("sshfrom")
                        .help("from= pattern-list")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SSHCOMMAND")
                        .long("sshcommand")
                        .help("command= pattern")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SSHOPTION")
                        .long("sshoption")
                        .help("additional options, e.g `no-pty`")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVER")
                        .long("server")
                        .help("Server name")
                        .num_args(1),
                )
                .arg(
                    Arg::new("SERVERGROUP")
                        .conflicts_with("SERVER")
                        .long("servergroup")
                        .help("Server group name")
                        .num_args(1),
                )
                .arg(Arg::new("COMMENT").long("comment").num_args(1)),
            Command::new("write-serverauth")
                .about("Write authorized_keys to workdir")
                .override_help("oerec-write-serverauth
Write authorized_keys to workdir

Usage: oerec write-serverauth [OPTIONS] --workdir <WORKDIR>

Options:
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
                        .num_args(1),
                )
                .arg(
                    Arg::new("FORCE")
                        .long("force")
                        .action(ArgAction::SetTrue)
                        .help("Overwrite workdir contents (USE WITH CAUTION)")
                ),
        ])
}
