
DATABASE_URL ?= postgres://postgres:postgres@localhost:5432/postgres

migration:
	sqlx migrate run --database-url $(DATABASE_URL) --source ./migrations
