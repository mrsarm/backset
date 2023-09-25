# Backset Service

> ### 🚧  Work in progress, just a few endpoints implemented.

Backset Service, or `bss`, it's a REST service to store elements, sets,
and relationships between them.

All objects are stored within a "tenant", and the tenant id is at
the beginning of all request URIs to identify to which tenant the
object belongs to.

### 🔋 Features

- Rest API with [Actix](https://github.com/actix/actix) and **Rust** 🦀.
- Storage in **Postgres** with [SQLx](https://github.com/launchbadge/sqlx).
  - SQL migrations.
- Configuration through environment variables.
- Logging configured with settings examples in the source.
- Tests and CI pipeline with GitHub Actions.
- Model driven design.
- Good serialization and error management:
  - It uses the crates: [Serde](https://serde.rs/), [validator](https://github.com/Keats/validator),
    [actix-web-validator](https://github.com/rambler-digital-solutions/actix-web-validator), [serde_valid](https://github.com/yassun7010/serde_valid) and
    [actix-contrib-rest](https://github.com/mrsarm/rust-actix-contrib-rest).
- Dockerized, ready to use for development or production envs.


### 🛠 Installation & Getting Started

You need Rust and Postgres to build and launch the app. Here are some shortcuts,
for more options see the [Installation](docs/installation.md) page.

```shell
# Install SQLx
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# Create a .env file (edit SQL connection and whatever)
cp .env.example .env

# Run Postgres, you can do so with Docker Compose
docker compose up postgres -d

# Create database
sqlx database create
# ^ replace "create" with drop to delete the DB 

# Migrate
sqlx migrate run

# Run the server (and compile if needed)
cargo run -- run
# `cargo test' to run the tests

# Check is alive with
curl localhost:8558/

# This command run backset in "command" mode, listing all tenants within the terminal
cargo run -- list tenants

# If you have already installed the binary with `make release && sudo make install',
# you can use the binary as follow to run the server:
backset run

# List tenants
backset list tenants
```

### 🐴 Endpoints usage

List of endpoints with usage samples: [API](docs/api.md).

### 🐳 Docker

Dockerfile and compose files are provided, check-out the [Docker](docs/docker.md) page.

### TO-DOs

Missing:

- [x] "/tenants" model and endpoints
- [ ] "/elements" model and endpoints
  - [x] `GET`
  - [ ] `GET` all
  - [x] `POST`
  - [x] `GET`
  - [x] `DELETE`
  - [ ] `PUT`
- [ ] "/sets" model and endpoints

More stuff to add:
- [ ] Random PKs (when more models are added).
- [ ] `/health?db=true&timeout=MILLIS`
      and `backset health --db --timeout MILLIS` to check DB connection as well.
      - [ ] The DB check could be in a new method `AppState::check_conn(Option<Duration>)`.


### About

**Project Home**: https://github.com/mrsarm/backset.

**Author**: Mariano Ruiz (mrsarm at gmail.com).

**License**: (2023) [Apache Software License 2.0](https://www.apache.org/licenses/LICENSE-2.0).
