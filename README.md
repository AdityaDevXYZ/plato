# PLATO

Project PLATO is a modular monolith built in Rust.

## Architecture

- `apps/`: Contains executable applications (e.g., API, worker, CLI).
- `crates/`: Contains domain and utility libraries.
- `docs/`: Documentation.
- `scripts/`: Development and deployment scripts.
- `tests/`: Integration tests.

## Build and Run

### How to Build
To build the entire workspace including all applications and crates:
```bash
cargo build --workspace
```
For production build:
```bash
cargo build --release --workspace
```

### How to Run
To run the main API application:
```bash
cargo run --bin api
```

### Environment Variables
Configuration is handled via `.env` file and system environment variables using `dotenvy` and `config`.
Required environment variables are documented in `.env.example`. 
Copy it to start:
```bash
cp .env.example .env
```
