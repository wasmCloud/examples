# Todo-sql

This actor implements the [Todo Backend Spec](https://github.com/TodoBackend/todo-backend-js-spec/blob/master/js/specs.js).

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability (with TLS enabled),
the relational database capability ('wasmcloud:sqldb') 
the number generator capability ('wasmcloud:builtin:numbergen')
and the logging capability (`wasmcloud:logging`). 
As usual, it is worth noting that this actor does _not_ know where its HTTP server comes from,
nor does it know which relational database implementation the host runtime has provided.

## Step 1 : Getting Started

- Clone the repo using either HTTPS or over SSH.
- Navigate to `/examples/actor/todo-sql/`.
- Make sure you have Docker installed. Follow the guide [here](https://docs.docker.com/get-docker/). Provide Docker with the necessary permissions to access the examples folder. You can configure shared paths from Docker -> Preferences... -> Resources -> File Sharing.

## Step 2 : To Generate Self-Signed TLS Certificates

The configuration file has TLS enabled, so you will need to generate self-signed TLS certificates,
if you don't already have certificates to use.
- Install mkcert from [here](https://github.com/FiloSottile/mkcert). Read more about it on that page.
- Run `mkcert example.com "*.example.com" example.test localhost 127.0.0.1 ::1` 
  - this command generates 'example.com+5.pem' and 'example.com+5-key.pem'
- Edit `http-linkdef.json` and set the **absolute paths** to these two files for `cert_file` and `priv_key_file`, respectively
  - `cert_file: "/path/to/example.com+5.pem"`
  - `priv_key_file: "/path/to/example.com+5-key.pem"`
- You may edit the default https port from 9000 to any other free port in this file.

## Step 3 : Running Registry and Postgres Containers
- Run the registry container by running `docker run -d -p 127.0.0.1:5000:5000 registry:2`. This port forwards the internal port to port 5000.
- Now let's set up a postgres database with a user that has write access. In the following example, we are setting up a user `postgres` with the same password. A dummy table is created in this database such using the `create-table.sql` script. Edit the path to the `create-table.sql` file in the following command.

```
docker run --rm -it \
  --name pgdb \
  -p 127.0.0.1:5432:5432 \
  -e POSTGRES_PASSWORD=postgres \
  -v /path/to/examples/actor/todo-sql/create-table.sql:/docker-entrypoint-initdb.d/create-table.sql \
  postgres:13

```
- Make sure that the ports being used for these containers are free to use.
- Edit `sql-linkdef.json` and update the uri field with the credentials and host/port. This is how your file should look like:

```
{
  "uri": "postgresql://postgres:postgres@127.0.0.1:5432/postgres",
  "pool": {
    "max_connections": 2,
    "min_idle": 1,
    "max_lifetime_secs": 300,
    "idle_timeout_secs": 300
  }
}
```

- Postgres URI is structured in the following manner:
```
postgres[ql]://[username[:password]@][host[:port],]/database[?parameter_list]

\_____________/\____________________/\____________/\_______/\_______________/
     |                   |                  |          |            |
     |- schema           |- userspec        |          |            |- parameter list
                                            |          |
                                            |          |- database name
                                            |
                                            |- hostspec
```

## Step 3 -  Running wasmCloud and NATS
- Follow the guide for installing [wasmCloud](https://wasmcloud.dev/overview/installation/) and [NATS](https://docs.nats.io/running-a-nats-service/introduction/installation).
- Also make sure that you have [jq](https://stedolan.github.io/jq/) as well as Rust installed.
- Run the wasmCloud host by running `wash up --allowed-insecure localhost:5000,127.0.0.1:5000` since we are using a self-signed certificate.
- Navigate to `localhost:4000` to view the wasmCloud dashboard.

## Step 4 - Run the Makefile and Tests
- Run `make clean-start` from the todo-sql folder. This will push the actor, start the providers, create the necessary linkdefs and link the components with each other. This is how your wasmCloud dashboard should look like:
![wasmCloud Dashboard](images/wasmcloud-dashboard.png)
- Open your browser to "https://localhost:9000/", and accept the warning from the browser that the server is using a self-signed certificate.
- Let's run the test suite from www.todobackend.com by navigating to "https://localhost:9000/test/" and clicking on the green button. You should see the tests cleared: 
![Tests cleared on localhost:9000](images/tests-passed.png)
- Congratulations you have run the postgres example successfully.

## Troubleshooting
- Make sure all the ports that are being used are open : 4000, 5000, 5432 and 9000.
- Docker should have permission to access the examples folder and its subdirectories as it needs access to the `create-tables.sql file`.
- Check the paths in `http-linkdef.json` and uri in `sql-linkdef.json`. Check if your postgres database is working in the specific docker container and that the correct user has all the necessary permissions.
- `wash up` uses port 4000 as of now. If the host is unable to start, inspect this port and kill the necessary processes before restarting. `wash down` will gracefully shut down the host as well.
- Check the base64 command on your terminal and the flag it needs to accept an input file. It may be a `-w0` or `-i` flag. If this causes any issues for you during `make clean-start` you may edit the Makefile in link-providers subcommand to use the base64 command with the necessary flag. For example: `base64 -i http-linkdef.json 2>/dev/null`. 