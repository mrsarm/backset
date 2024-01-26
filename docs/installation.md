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
RUSTFLAGS='-C strip=symbols' cargo install sqlx-cli --no-default-features --features native-tls,postgres
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

#### Revert last migration

```shell
sqlx migrate revert
```

#### Add new migration script

```shell
sqlx migrate add -r <name-migration>
```


#### Create and migrate DB for tests

The `DATABASE_URL` has to end with "_test", or backset will automatically
append the "_test" string to the name when connecting to the DB for testing.

you can execute the `./scripts/recreate-db-test.sh` to create or re-create the
DB for `cargo test`, or execute the re-creation and the tests with:

```shell
make test
```

### Build

Debug build:

```shell
cargo build
```

Or just `make` that will also compile tests.

For an optimized production build:

```shell
cargo build --release
```

Or `make release`.

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
cargo run -- run
```

Or `make run`, or `./target/debug/backset` if it's already compiled in debug mode (default),
or to run the production build with the "production" setup:

```shell
APP_ENV=production ./target/release/backset run
```

See the [Docker](docker.md) page to see how to build and run with Docker.

### Install

To install the optimized binary for release under the `/usr/local/bin/` folder:

```shell
make release
sudo make install
```

To change the prefix folder use the `INSTALL_PREFIX` env variable. E.g. to
install the binary under the user folder at `~/.local/bin/backset`:

```shell
make release
INSTALL_PREFIX="$HOME/.local" make install
```

Uninstall with:

```shell
make uninstall
```

You can use the same variable `INSTALL_PREFIX` to set the base directory
where the binary is installed.

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

### Environment variables

Check out [.env.example](../.env.example) that has some examples, and the full list
of variables in the [compose.yaml](../compose.yaml), in the service `bss`,
section `environment`.
