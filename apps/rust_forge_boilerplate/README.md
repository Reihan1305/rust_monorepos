# 🦀 Rust Forge Boilerplate

**Monorepo modular Rust dengan Actix-web framework** - Production-ready boilerplate untuk membangun scalable REST API dengan arsitektur yang bersih dan maintainable.

## ✨ Features

- 🚀 **5 Entry Points**: Server, Worker, Scheduler, Migrator, Seeder
- 🧩 **Modular Architecture**: High modularity, low coupling, high cohesion
- 🔒 **Type-Safe**: Compile-time checked SQL queries dengan SQLx
- ⚡ **High Performance**: Actix-web + Tokio async runtime
- 🎯 **Local Middleware**: Validasi & sanitasi per module (bukan global)
- 📦 **Multi-Database**: PostgreSQL, MongoDB, Redis support
- 🔄 **Background Jobs**: Worker & Scheduler untuk async tasks
- 📝 **Structured Logging**: Tracing untuk observability
- ✅ **Input Validation**: Validator untuk request validation
- 🛠️ **Developer Friendly**: Hot reload, clear error messages

## 📚 Documentation

- **[QUICKSTART.md](./QUICKSTART.md)** - Getting started guide
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Detailed architecture documentation
- **[STRUCTURE.md](./STRUCTURE.md)** - Project structure & file responsibilities
- **[deployment/README.md](./deployment/README.md)** - Docker deployment guide

## 🏗️ Architecture Overview

### Entry Points (5 Wajib)

| Binary | Purpose | Command |
|--------|---------|---------|
| **server** | HTTP REST API | `cargo run --bin server` |
| **worker** | Background jobs | `cargo run --bin worker` |
| **scheduler** | Cron tasks | `cargo run --bin scheduler` |
| **migrator** | DB migrations | `cargo run --bin migrator` |
| **seeder** | DB seeding | `cargo run --bin seeder` |

### Module Structure

Each module must follow this structure with `_modules` suffix:

```
{module_name}_modules/
├── dto.rs          # Data Transfer Objects
├── middleware.rs   # LOCAL validation & sanitization
├── handler.rs      # HTTP handlers
├── service.rs      # Business logic
├── repository.rs   # Database access (optional)
└── mod.rs          # Module exports & routes
```

**Naming Convention**: Module folders must end with `_modules` (e.g., `user_modules`, `product_modules`, `auth_modules`)

### Layer Flow

```
Request → Routes → Handler → Middleware → Service → Repository → Database
            ↓         ↓                       ↓
         Routing   Business              Data Access
                    Logic               & Transform
```

**Responsibilities:**
- **Handler**: Business logic, orchestration, decisions
- **Service**: Data fetching, validation, transformation
- **Repository**: Raw database queries

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- PostgreSQL 14+
- Redis 6+
- MongoDB 5+ (optional)

### Setup

```bash
# 1. Copy environment file
cp .env.example .env

# 2. Update .env dengan database credentials

# 3. Build project
cargo build

# 4. Run migrations
cargo run --bin migrator

# 5. Start server
cargo run --bin server
```

Server will run at `http://127.0.0.1:8080`

### Test API

```bash
# Health check
curl http://localhost:8080/health

# Create user
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","email":"john@example.com"}'

# List users
curl http://localhost:8080/users
```

## 📦 Tech Stack

| Category | Technology |
|----------|-----------|
| **Framework** | Actix-web 4.x |
| **Runtime** | Tokio |
| **Database** | PostgreSQL (SQLx), MongoDB |
| **Cache** | Redis |
| **Validation** | Validator |
| **Logging** | Tracing + tracing-subscriber |
| **Scheduler** | tokio-cron-scheduler |
| **Config** | config-rs + dotenv |

## 🎯 Core Principles

### ✅ DO

- ✅ Local middleware per module
- ✅ Business logic in handler layer
- ✅ Data access and transformation in service layer
- ✅ Service validates and prepares data for handler
- ✅ Dependency injection via Actix Data
- ✅ Async/await for I/O operations

### ❌ DON'T

- ❌ Global middleware for domain logic
- ❌ Business logic in service (service is for data access)
- ❌ Direct database access from handler
- ❌ Domain logic in common library
- ❌ Blocking operations in async context
- ❌ Share middleware between modules

## 🧩 Creating New Module

### Quick Command

```bash
# Create module structure
mkdir -p product_modules
touch product_modules/{dto,middleware,handler,service,repository,mod}.rs
```

### Implementation Steps

1. **Implement files** (see `healthcheck_modules` as example)
2. **Register module** in `lib.rs`:
   ```rust
   pub mod product_modules;
   ```
3. **Register routes** in `cmd/server/main.rs`:
   ```rust
   use rust_forge_boilerplate::product_modules;
   // ...
   .configure(product_modules::configure_routes)
   ```

**Note**: The `healthcheck_modules` is a simple, production-ready example. You can start developing immediately without removing any code.

## 🗄️ Database Migrations

```bash
# Create migration
sqlx migrate add create_products_table

# Edit migration file in migrations/ folder

# Run migrations
cargo run --bin migrator
```

## 🔧 Development

### Hot Reload

```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
cargo watch -x 'run --bin server'
```

### Useful Commands

```bash
# Check code
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Build for production
cargo build --release
```

## 📊 Project Structure

```
rust_forge_boilerplate/
├── cmd/                  # Entry points (5 binaries)
│   ├── server/
│   ├── worker/
│   ├── scheduler/
│   ├── migrator/
│   └── seeder/
├── common/               # Shared utilities
│   ├── config.rs
│   ├── error.rs
│   └── infrastructure/
├── healthcheck_modules/  # Example: Health check module
│   ├── dto.rs
│   ├── handler.rs
│   └── mod.rs
├── migrations/           # Database migrations
├── config/               # Configuration files
├── lib.rs                # Library root
├── Cargo.toml            # Dependencies & binaries
└── Makefile              # Build shortcuts
```

## 🔐 Environment Variables

```bash
# Server
APP__SERVER__HOST=127.0.0.1
APP__SERVER__PORT=8080

# Database
APP__DATABASE__URL=postgresql://user:password@localhost:5432/db
APP__DATABASE__MAX_CONNECTIONS=10

# Redis
APP__REDIS__URL=redis://localhost:6379

# MongoDB
APP__MONGODB__URL=mongodb://localhost:27017
APP__MONGODB__DATABASE=rust_forge_db

# Logging
RUST_LOG=info
```

## 🚢 Production Deployment

```bash
# Build optimized binaries
cargo build --release

# Binaries akan ada di target/release/
# - server
# - worker
# - scheduler
# - migrator
# - seeder
```

## 📖 Example Module: Health Check

Module `healthcheck_modules` sudah include sebagai contoh implementasi:

- ✅ Health check endpoint (`/api/health`)
- ✅ Readiness check endpoint (`/api/ready`)
- ✅ Checks PostgreSQL, Redis, MongoDB connectivity
- ✅ Simple & ready to use
- ✅ No need to delete - useful for production monitoring

## 🤝 Contributing

Contributions welcome! Silakan buat module baru atau improve existing code.

## 📄 License

MIT License - feel free to use this boilerplate untuk project apapun.

## 🙏 Acknowledgments

Built with:
- [Actix-web](https://actix.rs/) - Fast web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [Tokio](https://tokio.rs/) - Async runtime

---

**Happy Coding! 🦀✨**
