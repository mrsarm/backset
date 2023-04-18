# Backset Service

> 🚧 Work in progress, just a few endpoints implemented

Backset Service, or `bss`, it's a REST service to store elements, sets,
and relations between the two.

All objects are stored within a "tenant", and the tenant id is at
the beginning of all request URIs to identify to which tenant the
object belongs to.

### 🔋 Features

- Rest API with **Actix** and **Rust** 🦀.
- Storage in **Postgres** with SQLx.
  - SQL migrations.
- Configuration through environment variables.
- Logging configured with settings examples in the source.
- CI pipeline with GitHub Actions.
- Model driven design.
- Good error management.
  - JSON validations with "validator" and "actix-web-validator" crates.
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

# Run (and compile if needed)
cargo run
# ^ replace "run" with "test" to run the tests

# Check is alive with
curl localhost:8558/health
```

### 🐴 Endpoints usage

List of endpoints with samples of usage: [API](docs/api.md).

### 🐳 Docker

Dockerfile and compose files are provided, check-out the [Docker](docs/docker.md) page.

### TO-DOs

Missing:

- [x] "/tenants" model and endpoints
- [ ] "/elements" model and endpoints
- [ ] "/sets" model and endpoints

More stuff to add:

- [x] Tests
- [x] Test/fix rollback implementation
- [ ] Random PKs
- [ ] Date / Datetime fields


### About

**Project Home**: https://github.com/mrsarm/backset.

**Author**: Mariano Ruiz (mrsarm at gmail.com).

**License**: (2023) [Apache Software License 2.0](https://www.apache.org/licenses/LICENSE-2.0).
