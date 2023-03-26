# Backset Service

Backset Service, or `bss`, it's a REST service to store elements, sets,
and relations between the two.

All objects are stored within a "tenant", and the tenant id is at
the beginning of all request URIs to identify to which tenant the
object belongs to.


## Installation & Getting Started

### Requirements

- Rust 2021 edition and Cargo (tested with Rust 1.67+, may work with older versions as well).
- Postgres or Docker and Docker Compose

### Migrations

Migrations are managed by SQLx, check-out the
[SQLx CLI](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md) docs
for more info, but here are some shortcuts:

#### Create DB

Create/drop the database at `DATABASE_URL`:

```shell
sqlx database create
sqlx database drop
```

#### Run migrations

```shell
sqlx migrate run
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

### Style and Formatting

```shell
cargo fmt -- --check
```

### Tests

(TO-DO)

```shell
cargo test
```

### Endpoints usage

All the examples are provided with *HTTPie*.

#### Health check

```shell
http :8000/health
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
http :8000/tenants --raw '{"name": "Tenant Name"}'
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
http :8000/tenants/17
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
  - [ ] DELETE /tenants/{id}
- [ ] "/elements" model and endpoints
- [ ] "/sets" model and endpoints

More stuff to add:

- [x] Input validations
- [ ] Some DB setups has to be configured with envs
- [ ] Serialize HTTP 500 responses with generic JSON errors
      and log errors with ERROR severity, e.g. DB connection errors 
- [ ] Random PKs
- [ ] Tests
- [ ] Docker image
