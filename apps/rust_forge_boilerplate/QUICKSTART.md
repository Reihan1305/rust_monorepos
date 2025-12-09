# Quick Start Guide

## Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- PostgreSQL 14+
- Redis 6+
- MongoDB 5+ (optional)

## Setup

### 1. Clone & Navigate
```bash
cd apps/rust_forge_boilerplate
```

### 2. Environment Configuration
```bash
cp .env.example .env
```

Edit `.env` with your database credentials:
```bash
APP__DATABASE__URL=postgresql://user:password@localhost:5432/rust_forge_db
APP__REDIS__URL=redis://localhost:6379
APP__MONGODB__URL=mongodb://localhost:27017
APP__MONGODB__DATABASE=rust_forge_db
```

### 3. Install Dependencies
```bash
cargo build
```

### 4. Setup Database
```bash
# Create database & run migrations
cargo run --bin migrator

# (Optional) Seed data
cargo run --bin seeder
```

### 5. Run Server
```bash
cargo run --bin server
```

Server will run at `http://127.0.0.1:8080`

### 6. Test API
```bash
# Health check
curl http://localhost:8080/health

# Create user
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","email":"john@example.com"}'

# List users
curl http://localhost:8080/users

# Get user by ID
curl http://localhost:8080/users/{user_id}

# Update user
curl -X PUT http://localhost:8080/users/{user_id} \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe"}'

# Delete user
curl -X DELETE http://localhost:8080/users/{user_id}
```

## Running Other Services

### Background Worker
```bash
cargo run --bin worker
```

### Scheduler (Cron Jobs)
```bash
cargo run --bin scheduler
```

## Development

### Create New Module

1. **Create folder structure:**
```bash
mkdir -p product_modules
touch product_modules/{dto,middleware,handler,service,repository,mod}.rs
```

2. **Implement files** (see `healthcheck_modules` as example)

3. **Register module:**
```rust
// lib.rs
pub mod product_modules;
```

4. **Register routes:**
```rust
// cmd/server/main.rs
use rust_forge_boilerplate::product_modules;
// ...
.configure(product_modules::configure_routes)
```

### Create Migration
```bash
sqlx migrate add create_products_table
```

Edit the file in `migrations/` folder, then run:
```bash
cargo run --bin migrator
```

### Hot Reload (Development)
Install cargo-watch:
```bash
cargo install cargo-watch
```

Run with auto-reload:
```bash
cargo watch -x 'run --bin server'
```

## Production Build

```bash
cargo build --release
```

Binaries will be in `target/release/`:
- `server`
- `worker`
- `scheduler`
- `migrator`
- `seeder`

## Docker Deployment

### Quick Start with Docker Compose

```bash
cd deployment
docker-compose up -d
```

This will start all services:
- PostgreSQL, Redis, MongoDB
- Server (http://localhost:8080)
- Worker, Scheduler
- Migrator (runs once)

### Check Status

```bash
docker-compose ps
docker-compose logs -f server
```

### Stop Services

```bash
docker-compose down
```

For detailed deployment guide, see [deployment/README.md](./deployment/README.md)

## Troubleshooting

### Database Connection Error
- Make sure PostgreSQL is running
- Check credentials in `.env`
- Test connection: `psql -U user -d rust_forge_db`

### Redis Connection Error
- Make sure Redis is running: `redis-cli ping`
- Check URL in `.env`

### Compilation Error
- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build`

## Next Steps

- Read [ARCHITECTURE.md](./ARCHITECTURE.md) for architecture details
- See `healthcheck_modules` as implementation example
- Implement authentication module
- Add logging & monitoring
- Setup CI/CD pipeline

## Useful Commands

```bash
# Check code
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Build documentation
cargo doc --open

# Clean build artifacts
cargo clean
```

## Resources

- [Actix-web Documentation](https://actix.rs/)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [Rust Book](https://doc.rust-lang.org/book/)
