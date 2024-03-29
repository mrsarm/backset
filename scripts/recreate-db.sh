#!/usr/bin/env sh

# Re-create the DB used for local environment, do NOT use it with a prod DB.

set -ex

DATABASE_URL="${DATABASE_URL:-postgresql://postgres:localhost@postgres:5432/bss_dev}"

sqlx database drop -y
sqlx database create
sqlx migrate run
