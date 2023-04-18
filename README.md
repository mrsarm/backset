# Backset Service

Backset Service, or `bss`, it's a REST service to store elements, sets,
and relations between the two.

All objects are stored within a "tenant", and the tenant id is at
the beginning of all request URIs to identify to which tenant the
object belongs to.

## Features

- Rest API with Actix
- Storage in Postgres with SQLx
  - SQL migrations
- Configuration through environment variables
- Logging
- CI pipeline with GitHub Actions
- Model driven design
- Good error management
  - JSON validations with "validator" and "actix-web-validator" crates


## Installation & Getting Started

### Requirements

- Rust 2021 edition and Cargo (tested with Rust 1.67+, may work with older versions as well).
- Postgres or Docker and Docker Compose

### Migrations

Migrations are managed by SQLx, check-out the
[SQLx CLI](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md) docs
for more info, but here are some shortcuts:

#### Install SQLx CLI

```shell
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

#### Create DB

Create/drop the database at `DATABASE_URL`:

```shell
sqlx database create
sqlx database drop
```

Or use `make recreate-db`.

#### Run migrations

```shell
sqlx migrate run
```

Or use `make migrate`.

#### Check migrations

```shell
sqlx migrate info
```


#### Create and migrate DB for tests

The `DATABASE_URL` has to end with "_test", or backset will automatically
append the "_test" string to the name when connecting to the DB for testing.

you can execute the `./scripts/recreate-db-test.sh` to create or re-create the
DB for `cargo test`, or execute the re-creation and the tests with:

```shell
make test
```

#### Update "offline mode" queries

```shell
cargo sqlx prepare
```

### Build

Debug build, with a Postgres DB instance with a migrated DB running:

```shell
cargo build
```

#### Build in "offline mode"

```shell
SQLX_OFFLINE=true cargo build
```

### Linting

```shell
cargo clippy -- -D warnings
```

Or `make lint`.

### Style and Formatting

```shell
cargo fmt -- --check
```

Or `make fmt-check`.

### Run

```shell
cargo run
```

Or `make run`, or `./target/debug/backset` if it's already compiled in debug mode (default),
or to run the production build with the "production" setup:

```shell
APP_ENV=production ./target/release/backset
```

See the [Docker](#docker) section to see how to build and run with Docker.

### Tests

Read the [Create and migrate DB for tests](#create-and-migrate-db-for-tests) section.

Execute with:

```shell
make test
```

To only execute the tests without recreating the test DB:

```shell
cargo test
```


### Endpoints usage

All the examples are provided with *HTTPie*.

#### Health check

To check the service is running (it does not check the DB connection).

```shell
http :8558/health
HTTP/1.1 200 OK
content-type: application/json
...

{
    "service": "backset",
    "status": "UP"
}
```

#### POST /tenants

```shell
http :8558/tenants --raw '{"name": "Tenant Name"}'
HTTP/1.1 201 Created
content-type: application/json
...

{
    "id": 17,
    "name": "Tenant Name"
}
```

#### GET /tenants/{id}

```shell
http :8558/tenants/17
HTTP/1.1 200 OK
content-type: application/json
...

{
    "id": 17,
    "name": "Tenant Name"
}
```

#### GET /tenants

Query arguments `page_size` (default 50) and `offset` (default 0) are optionals. 

```shell
$ http ":8558/tenants?page_size=5&offset=10"
HTTP/1.1 200 OK
content-type: application/json
...

{
    "data": [
        {
            "id": 12,
            "name": "Some Tenant"
        },
        {
            "id": 13,
            "name": "Another Tenant"
        },
        ...
    ],
    "offset": 10,
    "page_size": 5,
    "total": 28
}
```

#### DELETE /tenants/{id}

```shell
$ http DELETE :8558/tenants/8
HTTP/1.1 204 No Content
...
```


## Docker

A reference [Dockerfile](Dockerfile), also [compose.yml](compose.yml) and
[.env.example](.env.example) files are provided, you can run
all from here, bss and Postgres.

First, copy the `.env.example` file as `.env` file, and edit whatever
value you want to:

```shell
cp .env.example .env
```

Then before run for the first time the containers, you have to either
download the images from Docker Hub or build them from the source code. To
build the images from the source code, execute:

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

### Running commands inside bss the container

The bss image is a thin runtime for `backset` and the `sqlx`
command installed under the `/usr/app/bin` folder. You can
access to the container with:

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

## TO-DOs

Missing:

- [x] "/tenants" model and endpoints
- [ ] "/elements" model and endpoints
- [ ] "/sets" model and endpoints

More stuff to add:

- [x] Tests
- [x] Test/fix rollback implementation
- [ ] Random PKs
