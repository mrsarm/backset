#!/usr/bin/env sh

set -ex

DATABASE_URL="${DATABASE_URL:-postgresql://postgres:postgres@localhost:5433/bss_dev}"

DATABASE_URL="${DATABASE_URL}_test" sqlx database drop -y
DATABASE_URL="${DATABASE_URL}_test" sqlx database create
DATABASE_URL="${DATABASE_URL}_test" sqlx migrate run
