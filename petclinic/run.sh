#!/bin/sh
set -e

show_help() {
cat <<_SHOW_HELP
  This program runs the petclinic example. Useful commands:

  Basics:
   $0 all                          - Run Postgres and wasmCloud in Docker, start PetClinicapplication
   $0 dev                          - Run Postgres and wasmCloud in Docker, start locally built petclinic resources
   $0 wipe                         - stop everything and erase all secrets

  Database management
   $0 init-db                      - initialize app db user and tables
   $0 drop-db                      - remove app db user and tables

  Host/actor controls:
   $0 inventory                    - show host inventory

  Utility:
   $0 psql [ args ... ]            - start a psql cli as the app user
   $0 psql-root [ args ... ]       - start a psql cli as the db root user

_SHOW_HELP
}

## ---------------------------------------------------------------
## CONFIGURATION
##
# Providers
HTTPSERVER_REF=wasmcloud.azurecr.io/httpserver:0.16.1
SQLDB_REF=wasmcloud.azurecr.io/sqldb-postgres:0.3.1
HTTPSERVER_ID=VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M
SQLDB_POSTGRES_ID=VDJQVOMF5FI3S5P4KJLCK2N25525U4IQIPH6NLHWZVTRZXTU6AOX4OPN
# Actors
UI_REF=wasmcloud.azurecr.io/ui:0.3.2
UI_ID=MA2BF3WNKPR6LFEHOAEGVRWHPR7TS25AF5QTGSDKTDKHY4JV7DUEUSVG
CLINICAPI_REF=wasmcloud.azurecr.io/clinicapi:0.3.1
CLINICAPI_ID=MA5DZLFF733IR7TIDMBNOUMDS7I32NFUJIZ7LBSS5ED3V6GPFTDZJXZ3
VETS_REF=wasmcloud.azurecr.io/vets:0.3.1
VETS_ID=MBZ3XLCME3RZBF7GQJ2LNIFZXHOGEJ2I7GVRHSIOPERWQXGMAH5NCI4F
VISITS_REF=wasmcloud.azurecr.io/visits:0.3.1
VISITS_ID=MAGZWZ4POOV6VKPJEIYP477KAHBPX5TULKVDHLJQT5QYOARP6CNHFNKJ
CUSTOMERS_REF=wasmcloud.azurecr.io/customers:0.3.1
CUSTOMERS_ID=MATDCJMSWC4R3Q4CKWLZ6SSF67CPST5TIHP6773OCDJL4B52NS3CPTGU
# the registry using container name
REG_SERVER=registry:5000

# actor to link to httpserver. there can be only one since there's one listen port
HTTP_ACTOR=actors/clinicapi
# actors to link to sqldb-postgres
SQLDB_ACTORS="actors/customers actors/vets actors/visits"
WASMCLOUD_LATTICE_PREFIX=default

DB_HOST=127.0.0.1
DB_PORT=5432
WASMCLOUD_PORT=4000
DB_ROOT_USER=postgres
LOG_FILE=docker/logs.txt

APP_DB_NAME=petclinic
APP_DB_USER=petclinic
# name of the docker container running database
APP_DB_HOST=db
APP_INIT_SQL=db/tables.sql
APP_FAKE_DATA_SQL=./create_data.sql

# http configuration file. use https_config.json to enable TLS
HTTP_CONFIG=http_config.json
# sqldb config file. This is generated from the template
SQL_CONFIG=sql_config.json
SQL_CONFIG_TEMPLATE=sql_config.template

# command to generate passwords
MKPASS=uuidgen
# command to base64 encode stdin to stdout
BASE64_ENC=base64

# uncomment this line to echo all database commands to terminal
PSQL_VERBOSE=-e

COMPOSE_FILE=./docker/docker-compose.yml
# where passwords are stored after being generated
SECRETS=.secrets
PSQL_ROOT=.psql_root
PSQL_APP=.psql_app
CREATE_APP_SQL=.create_app.sql
CLUSTER_SEED=.cluster.nk
WADM_FILE=populated_wadm.yaml

ALL_SECRET_FILES="$SECRETS $PSQL_ROOT $PSQL_APP $SQL_CONFIG $CREATE_APP_SQL $CLUSTER_SEED"

##
## END CONFIGURATION
## ---------------------------------------------------------------

# stop docker and wipe all data (database, nats cache, host provider/actors, etc.)
wipe_all() {

    cat >$SECRETS <<__WIPE
POSTGRES_PASSWORD=
WASMCLOUD_CLUSTER_SEED=
POSTGRES_PASSWORD=
WASMCLOUD_CLUSTER_SEED=
__WIPE

    docker compose --env-file $SECRETS -f $COMPOSE_FILE down
    wash drain all

    rm -f $ALL_SECRET_FILES .pgadmin_init.json $LOG_FILE
    rm -f $WADM_FILE
}

create_seed() {
    local _seed_type=$1
    wash keys gen -o json $_seed_type | jq -r '.seed'
}

create_secrets() {
    root_pass=$($MKPASS)
    app_pass=$($MKPASS)

    cluster_seed=$(create_seed Cluster)
    echo $cluster_seed >$CLUSTER_SEED

cat > $SECRETS << __SECRETS
POSTGRES_PASSWORD=$root_pass
WASMCLOUD_CLUSTER_SEED=$cluster_seed
__SECRETS


# hostname:port:database:username:password
cat > $PSQL_ROOT << __PSQL_ROOT
$DB_HOST:$DB_PORT:postgres:$DB_ROOT_USER:$root_pass
__PSQL_ROOT

# hostname:port:database:username:password
cat > $PSQL_APP << __PSQL_APP
$DB_HOST:$DB_PORT:$APP_DB_NAME:$APP_DB_USER:$app_pass
__PSQL_APP

# Save connection string actors use to connect to db
#
# We can't just use 'sed' as that would expose the password in the process table,
# note that there is no comma appended to the line, since we don't know if
# the sql line is at the end of the json file. Instead, let the template put the
# comma(s) in the right place
cat > $SQL_CONFIG << __SQL_CONFIG
$(awk 'BEGIN{p=0;} /DB_URI/{p=1;} p==0 {print;}' $SQL_CONFIG_TEMPLATE)
    "uri": "postgresql://$APP_DB_USER:$app_pass@$APP_DB_HOST:$DB_PORT/$APP_DB_NAME"
$(awk 'BEGIN {p=0;} p==1 {print;} /DB_URI/ {p=1;}' $SQL_CONFIG_TEMPLATE)
__SQL_CONFIG

cat > $CREATE_APP_SQL << __CREATE
CREATE USER $APP_DB_USER LOGIN PASSWORD '$app_pass';
CREATE DATABASE $APP_DB_NAME OWNER=$APP_DB_USER;
__CREATE

# create Server import file for pgadmin
cat > .pgadmin_init.json <<__PGADMIN_INIT
{
  "Servers": {
    "1": {
      "Name": "PetClinic",
      "Group": "Servers",
      "Host": "$APP_DB_HOST",
      "Port": 5432,
      "MaintenanceDB": "postgres",
      "Username": "$DB_ROOT_USER",
      "SSLMode": "prefer"
    }
  }
}
__PGADMIN_INIT

    # protect secret files
    chmod 600 $ALL_SECRET_FILES
}

# get the host id (requires wasmcloud to be running)
host_id() {
    wash ctl get hosts -o json | jq -r ".hosts[0].id"
}

# run psql cli as app user - convenience utility
psql_cli() {
    psql -X "postgresql://$APP_DB_USER@$DB_HOST:$DB_PORT/$APP_DB_NAME?passfile=$PSQL_APP" -w $@
}

# run psql cli as root user - convenience utility
psql_cli_root() {
    psql -X "postgresql://$DB_USER@$DB_HOST:$DB_PORT/postgres?passfile=$PSQL_ROOT" -w $@
}

wait_for_postgres() {
    # This might be overkill and could be replaced with a sleep
    # otherwise 'nc' would have to be on the required dependencies list
    until nc localhost $DB_PORT -w1 -z ; do
        echo Waiting for postgres to start ...
        sleep 1
    done
}

wait_for_wasmcloud() {
    # This might be overkill and could be replaced with a sleep
    # otherwise 'nc' would have to be on the required dependencies list
    until nc localhost $WASMCLOUD_PORT -w1 -z ; do
        echo Waiting for wasmCloud to start ...
        sleep 1
    done
}


# not idempotent, because 'create database' isn't.
# if you need to reinitialize the db use "run.sh drop-db; run.sh init-db"
init_db() {
    # as root user, create the app db and app user
    psql -X "postgresql://$DB_ROOT_USER@$DB_HOST:$DB_PORT?passfile=$PSQL_ROOT" \
        -w $PSQL_VERBOSE -f $CREATE_APP_SQL

    # as app user, create the app tables
    psql -X "postgresql://$APP_DB_USER@$DB_HOST:$DB_PORT/$APP_DB_NAME?passfile=$PSQL_APP" \
        -w $PSQL_VERBOSE -f $APP_INIT_SQL

		# Insert some data for vets and pettypes
		psql -X "postgresql://$APP_DB_USER@$DB_HOST:$DB_PORT/$APP_DB_NAME?passfile=$PSQL_APP" \
			-w $PSQL_VERBOSE -f $APP_FAKE_DATA_SQL
}

# drop the app database and user
# idempotent
drop_db() {

    psql -X "postgresql://$DB_ROOT_USER@$DB_HOST:$DB_PORT?passfile=$PSQL_ROOT" \
        -w -c '\x' -c "DROP DATABASE IF EXISTS $APP_DB_NAME;"

    psql -X "postgresql://$DB_ROOT_USER@$DB_HOST:$DB_PORT?passfile=$PSQL_ROOT" \
        -w -c '\x' -c "DROP USER IF EXISTS $APP_DB_USER;"
}


# start wasmcloud capability providers
# idempotent
start_providers() {
    local _host_id=$(host_id)

  	wash ctl start provider $HTTPSERVER_REF --link-name default --host-id $_host_id --timeout-ms 60000
	wash ctl start provider $SQLDB_REF      --link-name default --host-id $_host_id --timeout-ms 60000
}

# base-64 encode file into a string
b64_encode_file() {
    cat "$1" | $BASE64_ENC | tr -d ' \r\n'
}

show_inventory() {
    wash ctl get inventory $(host_id)
}

# check config files
check_files() {

    for f in $APP_INIT_SQL $HTTP_CONFIG $SQL_CONFIG; do
        if [ ! -f $f ]; then
            echo "missing file:$f"
            exit 1
        fi
    done

	# check syntax of json files
	jq < $HTTP_CONFIG >/dev/null
	jq < $SQL_CONFIG >/dev/null
}

##
# Petclinic scripts (local dev)
##
start_services_dev() {
    # ensure we have secrets
    if [ ! -f $SECRETS ]; then
        create_secrets
    fi
    docker compose --env-file $SECRETS -f $COMPOSE_FILE up -d db
    wait_for_postgres

    docker compose --env-file $SECRETS -f $COMPOSE_FILE --profile localdev up
    wait_for_wasmcloud
    # give things time to start
    sleep 5
}

# link actors with providers
# idempotent
link_providers_dev() {
    local _host_id=$(host_id)
    local _actor_id
    local _a

    # link gateway actor to http server
    _actor_id=$(make -C $HTTP_ACTOR --silent actor_id)
    wash ctl link put $_actor_id $HTTPSERVER_ID     \
        wasmcloud:httpserver config_b64=$(b64_encode_file $HTTP_CONFIG )

    # link actors to sqldb-postgres
    for _a in $SQLDB_ACTORS; do
        _actor_id=$(make -C $_a --silent actor_id)
	    wash ctl link put $_actor_id $SQLDB_POSTGRES_ID \
            wasmcloud:sqldb config_b64=$(b64_encode_file $SQL_CONFIG )
    done
}

run_dev() {
    # make sure we have all prerequisites installed
    ./checkup.sh

    if [ ! -f $SECRETS ]; then
        create_secrets
    fi
    check_files

    # start all the containers and initialize database
    start_services_dev
    init_db

    # build all actors
    make

    # push actors to registry
    make push

    # start actors
    make start REG_SERVER=registry:5000

    # link providers with actors
    link_providers_dev

		# Short sleep to let links settle in
		sleep 1

    # start capability providers: httpserver and sqldb 
    start_providers
}

##
# Petclinic scripts (published resources)
##

# start docker services
# idempotent
start_services() {
    # ensure we have secrets
    if [ ! -f $SECRETS ]; then
        create_secrets
    fi
    docker compose --env-file $SECRETS -f $COMPOSE_FILE up -d db
    wait_for_postgres

    docker compose --env-file $SECRETS -f $COMPOSE_FILE up > $LOG_FILE &
    wait_for_wasmcloud
    # give things time to start
    sleep 5
}

create_wadm_manifest() {
    password="$(cat $PSQL_APP | awk -F ':' '{ print $5}')"
    sed "s/user:pass@your.db.host.com/$APP_DB_USER:$password@db:$DB_PORT/g" wadm.yaml > $WADM_FILE
}

# idempotent
link_providers() {
    local _actor_id
    local _a

    # link gateway actor to http server
    wash ctl link put "$CLINICAPI_ID" $HTTPSERVER_ID -x $WASMCLOUD_LATTICE_PREFIX    \
        wasmcloud:httpserver config_b64="$(b64_encode_file $HTTP_CONFIG )"

    # link actors to sqldb-postgres
    SQLDB_ACTORS="$CUSTOMERS_ID $VISITS_ID $VETS_ID"
    for _a in $SQLDB_ACTORS; do
	    wash ctl link put "$_a" $SQLDB_POSTGRES_ID  -x $WASMCLOUD_LATTICE_PREFIX \
            wasmcloud:sqldb config_b64="$(b64_encode_file $SQL_CONFIG )"
    done
}

run_all() {
    # make sure we have all prerequisites installed
    ./checkup.sh

    if [ ! -f $SECRETS ]; then
        create_secrets
    fi
    check_files

    # start all the containers and initialize database
    start_services
    init_db

    # start actors
    wash ctl start actor $UI_REF --skip-wait
    wash ctl start actor $CLINICAPI_REF --skip-wait
    wash ctl start actor $CUSTOMERS_REF --skip-wait
    wash ctl start actor $VETS_REF --skip-wait
    wash ctl start actor $VISITS_REF --skip-wait

    # link providers with actors
    link_providers

		# Short sleep to let links settle in
		sleep 1

    # start capability providers: httpserver and sqldb 
    start_providers

    echo "PetClinic started successfully and is available at http://localhost:8080"
}

# NOTE(thomastaylor312): This mostly works, but we run into the race condition from
# https://github.com/wasmCloud/wasmcloud-otp/issues/361. So for now we are not exposing it. Once
# that bug fix is in, we can uncomment this

# run_wadm() {
#     ./checkup.sh

#     if [ ! -f $SECRETS ]; then
#         create_secrets
#     fi
#     check_files

#     # start all the containers and initialize database
#     start_services
#     init_db

#     create_wadm_manifest
#     wash app get petclinic 0.0.1 2>&1 > /dev/null || wash app put $WADM_FILE
#     wash app deploy petclinic 0.0.1

#     # Give a chance for providers to download and start
#     echo "Waiting for everything to start"
#     sleep 10

#     echo "PetClinic started successfully and is available at http://localhost:8080"
# }

case $1 in 

    secrets ) create_secrets ;;
    wipe ) wipe_all ;;
    start ) start_services ;;
    init-db ) init_db ;;
    drop-db ) drop_db ;;
    inventory ) show_inventory ;;
    start-providers ) start_providers ;;
    link-providers ) link_providers ;;
    run-all | all ) run_all ;;
    run-dev | dev ) run_dev ;;
    # wadm ) run_wadm ;;
    psql ) shift; psql_cli $@ ;;
    psql-root ) shift; psql_cli_root $@ ;;

    * ) show_help && exit 1 ;;

esac

