# Backset Service

Backset Service, or `bss`, it's a REST service to store elements, sets,
and relation between the two.

All objects are stored within a "tenant", and the tenant id is at
the beginning of all request URIs to identify to which tenant the
object belongs to.

## TO-TO

- [x] Only basic codebase working:
  - [x] Basic config from env variables and .env file
  - [x] Postgres connection and SQLX Migrations
  - [x] GET /health (`http :8000/health`)
  - [x] POST /tenants (`http :8000/tenants name="Me again"`)
  - [x] GET /tenants/{id}

Missing, still not migrated from Rocket codebase:

- [ ] SQLx offline mode doesn't work
- [ ] GET /tenants
- [ ] DELETE /tenants/<id>
- [-] Better error handling
- [x] DB code should be moved to a "db" module
  - [x] Each method should receive a transaction conn object,
        instead of the pool/connection object

More stuff to add:

- [ ] Better configuration management
- [ ] Startup should not fail if DB is not available
- [ ] Random PKs
- [ ] Tests
- [ ] Docker image
