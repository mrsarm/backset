# Docker

A reference [Dockerfile](../Dockerfile), also [compose.yml](../compose.yml) and
[.env.example](../.env.example) files are provided, you can run
all from here: bss and Postgres.

First, copy the `.env.example` file as `.env` file, and edit whatever
value you want to:

```shell
cp .env.example .env
```

Then before run for the first time the containers, you have to build
the backset image from the source code. To build it, execute:

```shell
docker compose build
```

### Up

Once the images are built in your local machine, create the containers
and run all of them with:

```shell
docker compose up
```

Just run Postgres with docker compose:

```shell
docker compose up postgres
```

Run just backset with docker:

```shell
docker run -it --rm --name bss --env-file=.env -p 8558:8558 mrsarm/backset
```

### Queries to Postgres

Run `psql` against the Postgres running with docker:

```shell
docker compose run psql
```

### Run migrations from containers

Execute any SQLx command with:

```shell
docker compose run sqlx migrate COMMAND [SUBCOMMAND]
```

E.g. `docker compose run sqlx migrate run` to run
the migrations, or `docker compose run sqlx migrate info`
to get the status of the migrations.

### Running commands inside the bss container

The bss image is a thin runtime for `backset` and the `sqlx`
command installed under the `/usr/app/bin` folder. You can
access to a running container within a shell session with:

```shell
docker exec -u root -it $(docker ps -f 'name=^bss$' -q) sh
```

To debug issues with the app or the container might be useful
to have some tools not installed by default, so first update
the packages indexes:

```shell
apk update
```

And then to install commands like `curl` and `jq`:

```shell
apk add curl jq
```

Any other tool available in Alpine is available for installation
in the container, e.g. the `netstat` command via the `net-tools` package.
