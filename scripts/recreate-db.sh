#!/usr/bin/env sh

# Re-create the DB used for local environment, do NOT use it with a prod DB.

set -ex

DATABASE_URL="${DATABASE_URL:-postgresql://postgres:postgres@postgres/bss_dev}"

cargo sqlx database drop -y
cargo sqlx database create
cargo sqlx migrate run
