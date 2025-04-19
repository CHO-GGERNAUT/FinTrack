DATABASE_URL ?= postgres://postgres:postgres@localhost:5432/postgres

BUILD_ENV = DATABASE_URL=$(DATABASE_URL)

prepare:
	cargo sqlx prepare --database-url $(DATABASE_URL)

build:
	${BUILD_ENV} cargo build

dev:
	${BUILD_ENV} cargo watch -x run

test:
	cargo test
