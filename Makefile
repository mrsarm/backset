.PHONY: clean build build-test release run list-tenants test lint fmt-check \
        migrate migrate-revert migrate-info create-db drop-db recreate-db recreate-db-test \
        check-sqlx-data psql
.DEFAULT_GOAL := build-all

clean:
	cargo clean

build:
	cargo build

build-test:
	cargo build --tests

build-all: build build-test

release:
	cargo build --release

run:
	cargo run -- run

list-tenants:
	cargo run -- list tenants

recreate-db:
	./scripts/recreate-db.sh

recreate-db-test:
	./scripts/recreate-db-test.sh

test: recreate-db-test
	RUST_LOG=warn cargo test

lint:
	cargo clippy -- -D warnings

fmt-check:
	cargo fmt -- --check

migrate:
	sqlx migrate run

migrate-revert:
	sqlx migrate revert

migrate-info:
	sqlx migrate info

create-db:
	sqlx database create

drop-db:
	sqlx database drop -y

sqlx-data:
	cargo sqlx prepare

check-sqlx-data:
	echo "Checking .sqlx/ is up to date with DB ..."
	cargo sqlx prepare
	git diff --exit-code --stat .sqlx/

psql:
	$(eval DATABASE_URL ?= $(shell cat .env | grep DATABASE_URL | cut -d= -f2))
	psql "${DATABASE_URL}"
