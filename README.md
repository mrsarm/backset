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
- [ ] "/sets" model and endpoints

More stuff to add:
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
- [ ] New CLI command `env` to print all configurations in the form `ENV=value`.
      - [ ] Encapsulates env access into a new module that prevent access to env
            not defined beforehand.
  - [ ] Fix: HTTP 500 errors are logged with `INFO` severity, it should be `ERROR`, e.g.
        when DB isn't available and a request is made, although an `ERROR` is printed out
        with enough details from the `backset:errors` module, another message
        from the logger `actix_web::middleware::logger` follows
        with the details of the failed request and the HTTP 500 response, but with
        `INFO` severity, making it hard to relate each other with log analysis tools. Note
        that the logger looks to not be configurable in this sense (`.wrap(Logger::default())`),
        so the solution looks to be a more robust logger with telemetry built-in like
        https://crates.io/crates/tracing-actix-web , although it doesn't look to log properly,
        or copy & paste the code of `actix_web.middleware.logger.Logger` and customize it to
        support filter by HTTP code and change to support logging levels.


### About

**Project Home**: https://github.com/mrsarm/backset.

**Author**: Mariano Ruiz (mrsarm at gmail.com).

**License**: (2023) [Apache Software License 2.0](https://www.apache.org/licenses/LICENSE-2.0).
