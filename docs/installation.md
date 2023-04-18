# Installation & Getting Started

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

See the [Docker](docker.md) page to see how to build and run with Docker.


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
