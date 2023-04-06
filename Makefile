.PHONY: clean build release run test lint fmt-check migrate create-db drop-db recreate-db recreate-db-test check-sqlx-data
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

check-sqlx-data:
	echo "Checking sqlx-data.json is up to date with DB ..."
	cp sqlx-data.json sqlx-data-prev.json
	cargo sqlx prepare
	diff sqlx-data-prev.json sqlx-data.json
