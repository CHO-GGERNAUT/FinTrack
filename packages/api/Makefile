DATABASE_URL ?= postgres://postgres:postgres@localhost:5432/postgres

BUILD_ENV = DATABASE_URL=$(DATABASE_URL)

migration:
	sqlx migrate run --database-url $(DATABASE_URL) --source ./migrations

prepare:
	cargo sqlx prepare --database-url $(DATABASE_URL) --workspace

build:
	${BUILD_ENV} cargo build

run:
	${BUILD_ENV} cargo watch -x run

test:
	cargo test
