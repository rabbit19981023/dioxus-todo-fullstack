1. Run postgres via docker-compose:

```bash
docker compose up -d
```

2. Setup database with sqlx:

```bash
cargo install sqlx-cli
sqlx migrate run --database-url postgres://postgres:postgres@localhost:5432/postgres
```

3. Launch app with hot realod:

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres dx serve --hotpatch --package web
```
