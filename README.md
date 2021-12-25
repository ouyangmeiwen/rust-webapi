# Backend Implementation in Rust with Clean Architect

- Clean Architect
- Independent framework: I've tried actix-web, warp, ...
- Independent database: Support MySQL, PostgresSQL ...

## Get started
```bash
export DATABASE_URL='dev.db'

cargo install diesel_cli --no-default-features --features "postgres sqlite mysql"
diesel setup
diesel migration run
```

**Run the server**
- [actix-web](./src/apps/actix): `cargo run --bin actix`
- [warp](./src/apps/warp): `cargo run --bin warp`
- [grpc](./src/apps/warp): `cargo run --bin grpc`


```bash
curl http://127.0.0.1:8000/users
curl http://127.0.0.1:8000/
curl http://127.0.0.1:8000/health
```
