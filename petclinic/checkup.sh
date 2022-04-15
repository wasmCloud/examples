#!/bin/sh

# make sure you have all the prerequisites installed

check=$(printf '\xE2\x9C\x94\n' | iconv -f UTF-8)
missing=$(printf '\xE2\x9D\x8C\n' | iconv -f UTF-8)

has() {
    command -v $1 >/dev/null
}

check_command() {
    if $(has $1) ; then
        printf "$check $1 - ok\n"
        return 0
    else
        printf "$missing $1 - MISSING: $2\n"
        return 1
    fi
}

check_port() {
    local _port=$1
    local _host=${2:-127.0.0.1}

    if $(nc $_host $_port -w1 -z); then
        printf "Warning $_host:$_port appears to be in use. there may be a conflict\n"
        return 1
    fi
    printf "port $_host:$_port not in use\n"
}


make_version() {
    if ! $(make -v | grep "GNU Make 4." >/dev/null); then
        printf "$missing It is strongly recommended: GNU make 4 or greater. Makefiles may not work for you\n"
    fi
    return 0
}

check_ports() {
    if ! $(has nc); then
        echo "To check ports you need to have nc (netcat or gnu-netcat) installed"
        return 1
    fi
    check_port 5432 # postgres
    check_port 9999 # pgadmin
    check_port 4222 # nats
    check_port 6222 # nats
    check_port 8222 # nats
    check_port 5000 # registry
    check_port 4000 # washboard
    check_port 443  # httpsserver (https)
    check_port 8080 # httpsserver (http)
}

check_requirements() {

    local _ok=0

    check_command wash    "Please install wash.  see https://github.com/wasmcloud/wash/" || _ok=1
    check_command jq      "Please install jq." || _ok=1
    check_command make    "Please install make." || _ok=1
    check_command base64  "Please install base64." || _ok=1
    check_command uuidgen "Please install uuidgen." || _ok=1
    make_version || _ok=1
    check_command docker  "Please install docker" || _ok=1
    check_command docker-compose  "Please install docker-compose" || _ok=1
    check_command psql    "Please install psql" || _ok=1

    if ! $(has nc); then
        echo "Recommended but not required: install 'nc' (netcat or gnu-netcat) to check for port availability"
    fi

    if [ $_ok -eq 0 ]; then
        printf "$check Prerequisites are installed - you're good to go! $check $check $check\n"
    else
        printf "Some items were missing or outdated. Please install missing items and try again.\n"
        exit 1
    fi
}


case $1 in

    ports ) check_ports ;;
    * )     check_requirements ;;

esac
