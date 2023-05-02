.PHONY: clean build build-test release run test lint fmt-check \
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
	cargo run

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

check-sqlx-data:
	echo "Checking sqlx-data.json is up to date with DB ..."
	cp sqlx-data.json sqlx-data-prev.json
	cargo sqlx prepare
	diff sqlx-data-prev.json sqlx-data.json

psql:
	$(eval DATABASE_URL ?= $(shell cat .env | grep DATABASE_URL | cut -d= -f2))
	psql "${DATABASE_URL}"
