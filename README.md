# Rust Web App

## Setup & Building

```bash
cargo install cargo-watch
cargo build
```

## Run servers locally (Manually)

### App service

```bash
cargo watch -q -c -w src/ -w assets/ -w templates/ -C app-service -x run
```

visit <http://localhost:8000>

### Auth service

```bash
cargo watch -q -c -w src/ -w assets/ -C auth-service -x run
```

visit <http://localhost:3000>

## Run servers locally (Docker)

```bash
docker compose build
docker compose up
```

visit <http://localhost:8000> and <http://localhost:3000>
