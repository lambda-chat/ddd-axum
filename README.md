
# DDD-Axum

DDD and Axum.

## Prepare Database

```sh
docker compose -f postgres/docker-compose.yaml up -d
set -x DATABASEURL postgres://admin:admin@localhost:5432/postgres-db  # for fish
cargo install sqlx-cli --no-default-features --features postgres  # only once
sqlx migrate run
```

## Cleanup

```sh
docker compose -f postgres/docker-compose.yaml down
docker volume rm postgres_postgres-data
```

## Build executable

```sh
cargo build
cargo build --features testing
```

`testing` feature will mock the DB and its peripheral stuffs.

## Test crates

```sh
cargo test -p application
```
