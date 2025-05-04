DATABASE_URL ?= postgres://postgres:postgres@localhost:5432/postgres

DB ?= postgres
MIGRATE_SOURCE = ./src/infrastructure/persistence/${DB}/migrations
RUST_LOG ?= debug

BUILD_ENV = DATABASE_URL=$(DATABASE_URL) RUST_LOG=$(RUST_LOG) RUST_BACKTRACE=1

migration:
	sqlx migrate run --database-url $(DATABASE_URL) --source $(MIGRATE_SOURCE)

prepare:
	cargo sqlx prepare --database-url $(DATABASE_URL) --workspace

build:
	${BUILD_ENV} cargo build

run:
	${BUILD_ENV} cargo watch -x run

test:
	cargo test
