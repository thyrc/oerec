_comp_cmd_oerec__server() {
    echo "$(oerec list-server 2>/dev/null |sed -e '1,2d' | awk '{print $1}')"
}

_comp_cmd_oerec__email() {
    echo "$(oerec list-user 2>/dev/null |sed -e '1,2d' | awk '{print $1}')"
}

_oerec() {
    local cur prev opts
        COMPREPLY=()
        cur="${COMP_WORDS[COMP_CWORD]}"
        prev="${COMP_WORDS[COMP_CWORD-1]}"
        cmd=""
        opts=""

    case ${prev} in 
        oerec)
            local sub='add-server list-server update-server delete-server add-servergroup list-servergroup update-servergroup delete-servergroup add-server-to-servergroup add-servergroup-to-servergroup delete-server-from-servergroup delete-servergroup-from-servergroup add-user list-user update-user delete-user add-key list-key update-key delete-key add-usergroup list-usergroup update-usergroup delete-usergroup add-user-to-usergroup add-usergroup-to-usergroup delete-user-from-usergroup delete-usergroup-from-usergroup add-serveraccess list-serveraccess update-serveraccess delete-serveraccess add-useraccess list-useraccess delete-useraccess enable-dns disable-dns enable-server disable-server enable-user disable-user write-serverauth'
            COMPREPLY=( $(compgen -W "${sub}" -- "${cur}") )
            ;;
        add-server)
            opts="-h --server --ip --comment --disabled --enable-dns --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        list-server)
            opts="-e -h -j --server --ip --id --exact --json --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        update-server)
            opts="-h --server --newname --ip --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        delete-server)
            opts="-h --server --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        add-servergroup)
            opts="-h --servergroup --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        list-servergroup)
            opts="-e -h -j --server --ip --all --empty --exact --json --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        update-servergroup)
            opts="-h --servergroup --newname --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        delete-servergroup)
            opts="-h --servergroup --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        add-server-to-servergroup)
            opts="-h --server --servergroup --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        add-servergroup-to-servergroup)
            opts="-h --subgroup --supergroup --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        delete-server-from-servergroup)
            opts="-h --server --servergroup --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        delete-servergroup-from-servergroup)
            opts="-h --subgroup --supergroup --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        add-user)
            opts="-h --email --name --type --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        list-user)
            opts="-e -h -j --email --name --id --exact --json --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        update-user)
            opts="-h --email --newemail --name --type --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        delete-user)
            opts="-h --email --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        add-key)
            opts="-h --email --sshkey --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        list-key)
            opts="-h -j --email --fingerprint --id --with-key --json --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        update-key)
            opts="-h --id --sshkey --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        delete-key)
            opts="-h --id --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        add-usergroup)
            opts="-h --usergroup --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        list-usergroup)
            opts="-e -h -j --email --usergroup --empty --exact --json --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        update-usergroup)
            opts="-h --usergroup --newname --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        delete-usergroup)
            opts="-h --usergroup --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        add-user-to-usergroup)
            opts="-h --email --usergroup --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        add-usergroup-to-usergroup)
            opts="-h --subgroup --supergroup --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        delete-user-from-usergroup)
            opts="-h --email --usergroup --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        delete-usergroup-from-usergroup)
            opts="-h --subgroup --supergroup --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        add-serveraccess)
            opts="-h --serveraccess --sshuser --sshfrom --sshcommand --sshoption --server --servergroup --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        list-serveraccess)
            opts="-e -h -j --server --ip --sshuser --exact --json --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        update-serveraccess)
            opts="-h --serveraccess --newname --sshuser --sshfrom --sshcomment --sshoption --server --servergroup --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        delete-serveraccess)
            opts="-h --serveraccess --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        add-useraccess)
            opts="-h --email --usergroup --serveraccess --until --comment --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        list-useraccess)
            opts="-e -h -j --server --ip --email --sshuser --serveraccess --expired --disabled --exact --json --servergroup --usergroup --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        delete-useraccess)
            opts="-h --email --usergroup --serveraccess --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        enable-dns)
            opts="-h --server --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        disable-dns)
            opts="-h --server --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        enable-server)
            opts="-h --server --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        disable-server)
            opts="-h --server --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        enable-user)
            opts="-h --email --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        disable-user)
            opts="-h --email --confirm --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        write-serverauth)
            opts="-h --workdir --force --help"
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        --server | --servername)
            local s
            s="$(_comp_cmd_oerec__server)"
            COMPREPLY=( $(compgen -W "${s}" -- "${cur}") )
            return 0
            ;;
        --email)
            local e
            e="$(_comp_cmd_oerec__email)"
            COMPREPLY=( $(compgen -W "${e}" -- "${cur}") )
            return 0
            ;;
        --type)
            COMPREPLY=( $(compgen -W "AD tool external" -- "${cur}") )
            return 0
            ;;
    esac

}

complete -F _oerec oerec add-server list-server update-server delete-server add-servergroup list-servergroup update-servergroup delete-servergroup add-server-to-servergroup add-servergroup-to-servergroup delete-server-from-servergroup delete-servergroup-from-servergroup add-user list-user update-user delete-user add-key list-key update-key delete-key add-usergroup list-usergroup update-usergroup delete-usergroup add-user-to-usergroup add-usergroup-to-usergroup delete-user-from-usergroup delete-usergroup-from-usergroup add-serveraccess list-serveraccess update-serveraccess delete-serveraccess add-useraccess list-useraccess delete-useraccess enable-dns disable-dns enable-server disable-server enable-user disable-user write-serverauth
