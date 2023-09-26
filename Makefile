.PHONY: clean build build-test release install uninstall run list-tenants test lint fmt-check \
        migrate migrate-revert migrate-info create-db drop-db recreate-db recreate-db-test wait-pg \
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

install:
	$(eval INSTALL_PREFIX ?= /usr/local)
	mkdir -p "${INSTALL_PREFIX}/bin"
	cp "target/release/backset" "${INSTALL_PREFIX}/bin/backset"

uninstall:
	$(eval INSTALL_PREFIX ?= /usr/local)
	rm -f "${INSTALL_PREFIX}/bin/backset"

run:
	cargo run -- run

list-tenants:
	cargo run -- list tenants

health:
	cargo run -- health

recreate-db:
	./scripts/recreate-db.sh

recreate-db-test:
	./scripts/recreate-db-test.sh

wait-pg:
	$(eval POSTGRES_PORT ?= 5432)
	./scripts/wait-port.sh "${POSTGRES_PORT}"

wait-server:
	$(eval PORT ?= 8558)
	./scripts/wait-port.sh "${PORT}"

test: build-test recreate-db-test
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

psql:
	$(eval DATABASE_URL ?= $(shell cat .env | grep DATABASE_URL | cut -d= -f2))
	psql "${DATABASE_URL}"
