
# DDD-Axum

DDD and Axum.

## Prepare Database

```sh
set -x DATABASEURL postgres://admin:admin@localhost:5432/postgres-db
docker compose -f postgres up
cargo sqlx prepare
cargo sqlx migrate
```

## Build executable

```sh
cargo build
```

## Test crates

```sh
cargo test -p application
```
