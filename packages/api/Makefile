
DATABASE_URL ?= postgres://postgres:postgres@localhost:5432/postgres

BUILD_ENV = DATABASE_URL=$(DATABASE_URL)

prepare:
	cargo sqlx prepare --database-url $(DATABASE_URL)

execute:
	${BUILD_ENV} cargo run

build:
	${BUILD_ENV} cargo build