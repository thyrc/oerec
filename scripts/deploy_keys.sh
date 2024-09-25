#!/bin/bash

set -u
set -o pipefail

PATH=/usr/bin:/bin:${HOME}/bin

###########
# variables
###########

script_canon="$(readlink -m "$0")"

script_name="${0##*/}"
lock_file="/tmp/${script_name}.lock"

work_dir="/var/lib/oere"
log_dir="/var/log"
log_file="${log_dir}/oere_sync.log"

temp_dir="$(mktemp -d /tmp/${script_name}-XXXXXXXX)"

sync_all=0
sync_single="null"
quiet_sync=0

###############
# basic locking
###############

if (set -o noclobber; echo "$$" >"$lock_file") 2>/dev/null
then
    trap 'set +e; \
    rm -f "$lock_file"; \
    rm -rf "$temp_dir";' INT TERM EXIT
else
    >&2 echo "Failed to acquire lock: '$lock_file'."
    >&2 echo "Held by PID $(cat "$lock_file")"
    exit 1
fi

###########
# functions
###########

usage () {
    cat <<USE
USAGE:
    $script_name [--all]
    $script_name [--single <IP>]

OPTIONS:
    --all           forced sync to all managed servers
    --single <IP>   sync all users on single server w/ IP
USE
    exit
}

#########
# options
#########

while [[ "$#" -gt 0 ]]
do
    case "$1" in
        --help | -h )
            usage
            ;;
        --all | --force | -a )
            shift
            sync_all=1
            ;;
        --quiet | -q )
            shift
            quiet_sync=1
            ;;
        --single | --ip | --sync )
            shift
            sync_single="$1"
            shift
            ;;
        -- )
            shift
            break
            ;;
        * )
            usage
            ;;
    esac
done

#########
# logging
#########

if [[ "${quiet_sync}" -eq 1 ]]; then
    exec 2> >(tee -a ${log_file}) 1>>${log_file}
else
    exec &> >(tee -a ${log_file})
fi

###########
# fn main()
###########

pushd "$work_dir" &>/dev/null
if [[ "$?" -ne 0 ]]; then
    >&2 echo "Could not change to work dir."
    exit 1
fi

oerec write-serverauth --force --workdir=${work_dir}
if [[ "$?" -ne 0 ]]; then
    >&2 echo "Could write server auth files."
    exit 1
fi

git add .

if [[ "${sync_all}" -eq 1 ]]; then
    find . -name authorized_keys |sed -e 's#^\./##' >${temp_dir}/generated.files
elif [[ "${sync_single}" != "null" ]]; then
    find ${sync_single} -name authorized_keys |sed -e 's#^\./##' >${temp_dir}/generated.files
else
    git diff --name-only --staged --diff-filter=d >${temp_dir}/generated.files
fi

git diff --name-only --staged --diff-filter=D >${temp_dir}/deleted.files

if [[ "$(stat -c %s ${temp_dir}/generated.files)" -eq 0 && "$(stat -c %s ${temp_dir}/deleted.files)" -eq 0 ]]; then
    exit 0
fi

if [[ "$(stat -c %s ${temp_dir}/deleted.files)" -ne 0 ]]; then
    cat ${temp_dir}/deleted.files \
        |cut -d'/' -f1 \
        |sort -u \
        |while read -r server; do \
            if [[ ! -d "${work_dir}/${server}" ]]; then
                echo "[$(date --iso-8601=seconds)] deleting server '${server}' (sync disabled)"
                sed -e "/^$server/d" -i ${temp_dir}/deleted.files
            fi
        done

    if [[ "$(stat -c %s ${temp_dir}/deleted.files)" -ne 0 ]]; then
        touch ${temp_dir}/del.jobs
        cat ${temp_dir}/deleted.files \
            |tr '/' ' ' \
            |while read -r server user auth; do \
                if [[ "${server}" == "" ]]; then \
                    >&2 echo "Server not found."; \
                    continue; \
                fi; \
                if [[ "${user}" == "" ]]; then \
                    >&2 echo "User not found."; \
                    continue; \
                fi; \
                home=$(ssh -n root@${server} "grep '^${user}:' /etc/passwd |cut -d':' -f6" 2>/dev/null); \
                if [[ "${home}" == "" ]]; then \
                    >&2 echo "Home for ${user}@${server} not found."; \
                    continue; \
                fi; \
                echo -n "echo \"[\$(date --iso-8601=seconds)] deleting user ${user}@${server}\"; " >>${temp_dir}/del.jobs; \
                if [[ "${user}" == "root" ]]; then \
                    echo -n "set -e; echo \"put /root/.ssh/id_ed25519.pub ${home}/.ssh/authorized_keys.new\" |sftp -b - root@${server} >/dev/null; set +e; " >>${temp_dir}/del.jobs; \
                    echo -n "ssh -n root@${server} \"touch ${home}/.ssh/authorized_keys; cp -p ${home}/.ssh/authorized_keys ${home}/.ssh/authorized_keys~; cp ${home}/.ssh/authorized_keys.new ${home}/.ssh/authorized_keys; chown ${user}:${user} ${home}/.ssh/authorized_keys; chmod 0640 ${home}/.ssh/authorized_keys; rm ${home}/.ssh/authorized_keys.new\"; " >>${temp_dir}/del.jobs; \
                    echo -n "ssh -n root@${server} \"diff -u ${home}/.ssh/authorized_keys~ ${home}/.ssh/authorized_keys 2>/dev/null\"; " >>${temp_dir}/del.jobs; \
                else \
                    echo -n "ssh -n root@${server} \"touch ${home}/.ssh/authorized_keys; cp -p ${home}/.ssh/authorized_keys ${home}/.ssh/authorized_keys~; rm ${home}/.ssh/authorized_keys\"; " >>${temp_dir}/del.jobs; \
                fi; \
                echo "echo \"[\$(date --iso-8601=seconds)] deleting user ${user}@${server} done.\"; " >>${temp_dir}/del.jobs; \
                server=""; user=""; home=""; \
            done

        cat ${temp_dir}/del.jobs \
            |shuf \
            |parallel-sh -j4
    fi
fi

for auth_file in $(cat ${temp_dir}/generated.files); do
    if [[ -e "${auth_file}" && "$(stat -c %s ${auth_file})" -lt 127 ]]; then
        >&2 echo "Excluding '${auth_file}'. File too small."
        sed -e "/^${auth_file//\//\\\/}/d" -i ${temp_dir}/generated.files
    fi
done

# warm-up / set-up ControlPath
cat ${temp_dir}/generated.files \
    |cut -d'/' -f1 \
    |sort -u \
    |while read -r srv; do echo "ssh -n root@${srv} -o ControlMaster=auto -o ConnectionAttempts=2 'exit 0' &>/dev/null"; done \
    |parallel-sh -j8 &>/dev/null

touch ${temp_dir}/sync.jobs
cat ${temp_dir}/generated.files \
    |tr '/' ' ' \
    |while read -r server user auth; do \
        ssh root@${server} -O check &>/dev/null; \
        if [[ "$?" -ne 0 ]]; then \
            >&2 echo "ControlMaster for '${server}' not found."; \
            continue; \
        fi; \
        if [[ "${server}" == "" ]]; then \
            >&2 echo "Server not found."; \
            continue; \
        fi; \
        osuser=$(ssh -n root@${server} "grep '^${user}:' /etc/passwd |cut -d':' -f1" 2>/dev/null); \
        if [[ "${osuser}X" == "X" ]]; then \
            echo "[$(date --iso-8601=seconds)] adding OS user '${user}@${server}'";  \
            ssh -n root@${server} "adduser --disabled-password --force-badname --quiet --gecos '${user}' '${user}'"; \
            if [[ $? -ne 0 ]]; then \
                >&2 echo "Could not add OS user ${user}@${server}."; \
                continue; \
            fi; \
            ssh -n root@${server} "if ! grep -qse '^AllowUsers.*${user}.*$' /etc/ssh/sshd_config; then sed -e '/^AllowUsers/s|$|& ${user}|' -i /etc/ssh/sshd_config; fi"; \
            ssh -n root@${server} "if ! grep -qse '^AllowGroups.*${user}.*$' /etc/ssh/sshd_config; then sed -e '/^AllowGroups/s|$|& ${user}|' -i /etc/ssh/sshd_config; fi"; \
            ssh -n root@${server} "systemctl reload sshd.service &>/dev/null"; \
        fi; \
        home=$(ssh -n root@${server} "grep '^${user}:' /etc/passwd |cut -d':' -f6" 2>/dev/null); \
        if [[ "${home}" == "" ]]; then \
            >&2 echo "Home for ${user}@${server} not found."; \
            continue; \
        fi; \
        echo -n "echo \"[\$(date --iso-8601=seconds)] syncing ${user}@${server}\"; " >>${temp_dir}/sync.jobs; \
        echo -n "ssh -n root@${server} \"mkdir -p ${home}/.ssh; chown ${user}:${user} ${home}/.ssh; chmod 0700 ${home}/.ssh\"; " >>${temp_dir}/sync.jobs; \
        echo -n "set -e; echo \"put ./${server}/${user}/authorized_keys ${home}/.ssh/authorized_keys.new\" |sftp -b - root@${server} >/dev/null; set +e; " >>${temp_dir}/sync.jobs; \
        echo -n "ssh -n root@${server} \"touch ${home}/.ssh/authorized_keys; cp -p ${home}/.ssh/authorized_keys ${home}/.ssh/authorized_keys~; cp ${home}/.ssh/authorized_keys.new ${home}/.ssh/authorized_keys; chown ${user}:${user} ${home}/.ssh/authorized_keys; chmod 0640 ${home}/.ssh/authorized_keys; rm ${home}/.ssh/authorized_keys.new\"; " >>${temp_dir}/sync.jobs; \
        echo -n "ssh -n root@${server} \"diff -u ${home}/.ssh/authorized_keys~ ${home}/.ssh/authorized_keys 2>/dev/null\"; " >>${temp_dir}/sync.jobs; \
        echo "echo \"[\$(date --iso-8601=seconds)] syncing ${user}@${server} done.\"; " >>${temp_dir}/sync.jobs; \
        server=""; user=""; home=""; \
    done

cat ${temp_dir}/sync.jobs \
    |shuf \
    |parallel-sh -j8

git commit -q -m "${script_name}: autocommit" >/dev/null
# git push -q -f >/dev/null

popd >/dev/null

exit 0
