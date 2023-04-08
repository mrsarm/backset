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

## TO-DOs

Missing, or still not migrated from Rocket codebase:

- [ ] Tenants endpoints
  - [x] GET /tenants/{id}
  - [x] POST /tenants
  - [ ] GET /tenants
- [ ] "/elements" model and endpoints
- [ ] "/sets" model and endpoints

More stuff to add:

- [ ] Random PKs
- [ ] Docker image
