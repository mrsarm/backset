#!/usr/bin/env sh

set -ex

DATABASE_URL="${DATABASE_URL:-postgresql://postgres:postgres@postgres/bss_dev}"

DATABASE_URL="${DATABASE_URL}_test" cargo sqlx database drop -y
DATABASE_URL="${DATABASE_URL}_test" cargo sqlx database create
DATABASE_URL="${DATABASE_URL}_test" cargo sqlx migrate run
