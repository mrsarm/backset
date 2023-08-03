# Backset Service

> ### 🚧  Work in progress, just a few endpoints implemented.

Backset Service, or `bss`, it's a REST service to store elements, sets,
and relations between the two.

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
- Good error management.
  - JSON validations with [validator](https://github.com/Keats/validator) and
    [actix-web-validator](https://github.com/rambler-digital-solutions/actix-web-validator) crates.
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
# ^ replace "run" with "test" to run the tests

# Check is alive with
curl localhost:8558/health
```

### 🐴 Endpoints usage

List of endpoints with usage samples: [API](docs/api.md).

### 🐳 Docker

Dockerfile and compose files are provided, check-out the [Docker](docs/docker.md) page.

### TO-DOs

Missing:

- [x] "/tenants" model and endpoints
- [ ] "/elements" model and endpoints
- [ ] "/sets" model and endpoints

More stuff to add:
- [ ] Add argument `include_total=(true|false)` (default `true`) to include/omit total count
      that might be expensive sometimes.
- [ ] Improve validations: when a required field is not passed
      the error is ```{"error": "Json deserialize error: missing field `name` at line ..."}```
      instead of providing field details. Check-out the default string handler that return
      a string instead of JSON (sucks) but can be a good sample of how to iterate the errors
      at least for these cases: https://github.com/rambler-digital-solutions/actix-web-validator/blob/master/src/error.rs#L51-L55
      - Looks like it's not possible, and alternative is to apply a regex to the error, e.g.
        when the error is a "ActixValidatorError::JsonPayloadError(Deserialize)", and the message
        is ```Json deserialize error: missing field `FIELDNAME` at line \d+ column \d+```,
        the error type and field name can be inferred.
- [ ] Random PKs
- [ ] Command lines options:
    - [ ] To check server status pinging `/health` endpoint, so a health check script
      can be added to `Dockerfile`.
    - [x] To run "tasks" that have access to the AppState and business logic, so later it's
          the base to write tasks that can be scheduled with tools like K8s jobs or Cron.
          A example task could be one that lists all tenants in the standard output.
    - [ ] Option to run the migrations command, so there is no need to include the
      `sqlx` command to run them, reducing the size a lot.


### About

**Project Home**: https://github.com/mrsarm/backset.

**Author**: Mariano Ruiz (mrsarm at gmail.com).

**License**: (2023) [Apache Software License 2.0](https://www.apache.org/licenses/LICENSE-2.0).
