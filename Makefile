.PHONY: clean build release run test lint fmt-check migrate create-db drop-db recreate-db recreate-db-test
.DEFAULT_GOAL := run

clean:
	cargo clean

build:
	cargo build

release:
	cargo build --release

run:
	cargo run

recreate-db:
	./scripts/recreate-db.sh

recreate-db-test:
	./scripts/recreate-db-test.sh

test: recreate-db-test
	RUST_LOG=warn cargo test --test tenants

lint:
	cargo clippy -- -D warnings

fmt-check:
	cargo fmt -- --check

migrate:
	sqlx migrate run

create-db:
	sqlx database create

drop-db:
	sqlx database drop -y
