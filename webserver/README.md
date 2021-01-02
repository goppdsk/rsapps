# Web server

## Run server on local

```bash
cargo run --bin rsapps-webserver
```

## Run database container

```bash
# At a first time
docker-compose up -d --build

# Second time and so on
docker-compose up -d
```

## Migrate database

```bash
cargo install --version=0.2.0 sqlx-cli --no-default-features --features postgres
sqlx migrate run
```

## Save database infromation

```bash
cargo sqlx prepare -- --bin rsapps-webserver
```
