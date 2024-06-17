# Rust Web App

[![Build, Test and Deploy to Prod](https://github.com/acgetchell/live-bootcamp-project/actions/workflows/prod.yml/badge.svg)](https://github.com/acgetchell/live-bootcamp-project/actions/workflows/prod.yml)
[![rust-clippy analyze](https://github.com/acgetchell/live-bootcamp-project/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/acgetchell/live-bootcamp-project/actions/workflows/rust-clippy.yml)
[![codecov](https://codecov.io/gh/acgetchell/live-bootcamp-project/graph/badge.svg?token=thr2pXXnhf)](https://codecov.io/gh/acgetchell/live-bootcamp-project)

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
./docker.sh
```

visit <http://localhost:8000> and <http://localhost:3000>
