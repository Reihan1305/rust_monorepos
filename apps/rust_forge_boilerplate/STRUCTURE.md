# Project Structure

```
rust_forge_boilerplate/
│
├── Cargo.toml                 # Package manifest & dependencies
├── lib.rs                     # Library root
├── Makefile                   # Build shortcuts
├── README.md                  # Project overview
├── QUICKSTART.md             # Getting started guide
├── ARCHITECTURE.md           # Architecture documentation
├── STRUCTURE.md              # This file
│
├── .env.example              # Environment variables template
├── .gitignore                # Git ignore rules
│
├── config/
│   └── default.toml          # Default configuration
│
├── migrations/               # Database migrations (SQLx)
│   ├── .gitkeep
│   └── 20231201000000_create_users_table.sql
│
├── cmd/                      # Entry points (5 wajib)
│   ├── server/
│   │   └── main.rs           # HTTP server (Actix-web)
│   ├── worker/
│   │   └── main.rs           # Background job processor
│   ├── scheduler/
│   │   └── main.rs           # Cron task scheduler
│   ├── migrator/
│   │   └── main.rs           # Database migration runner
│   └── seeder/
│       └── main.rs           # Database seeder
│
├── common/                   # Shared utilities (NO domain logic)
│   ├── mod.rs
│   ├── config.rs             # Configuration management
│   ├── error.rs              # Error types & handling
│   ├── infrastructure/       # Infrastructure connections
│   │   ├── mod.rs
│   │   ├── database.rs       # PostgreSQL connection
│   │   ├── redis.rs          # Redis connection
│   │   └── mongodb.rs        # MongoDB connection
│   └── utils/                # General utilities
│       └── mod.rs
│
└── healthcheck_modules/      # Example: Health check module
    ├── mod.rs                # Module exports & route config
    ├── dto.rs                # Data Transfer Objects
    └── handler.rs            # HTTP handlers
```

## File Responsibilities

### Root Level

#### Cargo.toml
- Package metadata
- Dependencies
- Binary definitions (5 entry points)
- Workspace configuration

#### Makefile
- Build shortcuts
- Common commands
- Development helpers

#### .env.example
- Environment variable template
- Configuration examples
- No sensitive data

### config/

#### default.toml
- Default configuration values
- Can be overridden by environment variables
- Structure mirrors AppConfig struct

### migrations/

#### SQL Files
- Database schema changes
- Versioned by timestamp
- Run via migrator binary
- Format: `YYYYMMDDHHMMSS_description.sql`

### src/cmd/

#### server/main.rs
- Initialize Actix-web server
- Setup middleware (Logger, CORS)
- Register module routes
- Dependency injection setup

#### worker/main.rs
- Background job processor
- Redis queue consumer
- Async job execution
- Error handling & retry logic

#### scheduler/main.rs
- Cron-based task scheduler
- Periodic job execution
- Scheduled maintenance tasks

#### migrator/main.rs
- Database migration runner
- Create database if not exists
- Run pending migrations
- Migration status tracking

#### seeder/main.rs
- Database seeding
- Initial data population
- Test data generation

### common/

#### config.rs
- Configuration structs
- Environment variable parsing
- Config file loading
- Validation

#### error.rs
- AppError enum
- Error type definitions
- HTTP error responses
- Error conversion traits

#### infrastructure/database.rs
- PostgreSQL connection pool
- SQLx configuration
- Connection management

#### infrastructure/redis.rs
- Redis connection manager
- Connection pooling
- Error handling

#### infrastructure/mongodb.rs
- MongoDB client
- Database selection
- Connection management

### {module_name}_modules/

#### Module Structure (Example: healthcheck_modules/)

##### mod.rs
```rust
// Module exports
pub mod dto;
pub mod handler;
pub mod middleware;
pub mod repository;
pub mod service;

// Route configuration
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Register routes
}
```

##### dto.rs
```rust
// Request DTOs (with validation)
pub struct CreateUserDto { ... }
pub struct UpdateUserDto { ... }

// Response DTOs
pub struct UserResponseDto { ... }
```

##### middleware.rs
```rust
// LOCAL validation & sanitization
pub async fn validate_create_user(...) -> AppResult<CreateUserDto>
pub async fn validate_update_user(...) -> AppResult<UpdateUserDto>
pub fn validate_user_id(...) -> AppResult<Uuid>
```

##### handler.rs
```rust
// HTTP request handlers
pub async fn list_users(...) -> AppResult<HttpResponse>
pub async fn get_user(...) -> AppResult<HttpResponse>
pub async fn create_user(...) -> AppResult<HttpResponse>
pub async fn update_user(...) -> AppResult<HttpResponse>
pub async fn delete_user(...) -> AppResult<HttpResponse>
```

##### service.rs
```rust
// Business logic
pub async fn list_users(...) -> AppResult<Vec<UserResponseDto>>
pub async fn get_user(...) -> AppResult<UserResponseDto>
pub async fn create_user(...) -> AppResult<UserResponseDto>
pub async fn update_user(...) -> AppResult<UserResponseDto>
pub async fn delete_user(...) -> AppResult<()>
```

##### repository.rs
```rust
// Database access
pub struct User { ... }  // DB model

pub async fn find_all(...) -> Result<Vec<User>, sqlx::Error>
pub async fn find_by_id(...) -> Result<Option<User>, sqlx::Error>
pub async fn create(...) -> Result<User, sqlx::Error>
pub async fn update(...) -> Result<User, sqlx::Error>
pub async fn delete(...) -> Result<bool, sqlx::Error>
```

## Data Flow

### Request Flow
```
HTTP Request
    ↓
Routes (mod.rs)
    ↓
Handler (handler.rs)
    ↓
Middleware (middleware.rs) - Validation & Sanitization
    ↓
Service (service.rs) - Business Logic
    ↓
Repository (repository.rs) - Database Access
    ↓
Database
```

### Response Flow
```
Database
    ↓
Repository - Return DB Model
    ↓
Service - Transform to DTO, Business Validation
    ↓
Handler - HTTP Response
    ↓
Client
```

## Module Creation Checklist

Saat membuat module baru:

- [ ] Buat folder `src/modules/{module_name}/`
- [ ] Buat file `dto.rs` dengan request/response DTOs
- [ ] Buat file `middleware.rs` dengan validasi lokal
- [ ] Buat file `handler.rs` dengan HTTP handlers
- [ ] Buat file `service.rs` dengan business logic
- [ ] Buat file `repository.rs` dengan database access (optional)
- [ ] Buat file `mod.rs` dengan exports & route config
- [ ] Register di `src/modules/mod.rs`: `pub mod {module_name};`
- [ ] Register routes di `src/cmd/server/main.rs`: `.configure(modules::{module_name}::configure_routes)`
- [ ] Buat migration file jika perlu table baru
- [ ] Update seeder jika perlu initial data

## Naming Conventions

### Files
- Snake case: `user_service.rs`, `create_user.rs`
- Descriptive: `validate_email.rs`, `hash_password.rs`

### Modules
- Snake case: `user`, `auth`, `product`
- Singular form: `user` not `users`

### Structs
- Pascal case: `CreateUserDto`, `UserResponseDto`
- Suffix with type: `Dto`, `Error`, `Config`

### Functions
- Snake case: `create_user`, `validate_email`
- Verb-first: `get_user`, `update_user`, `delete_user`

### Constants
- Upper snake case: `MAX_CONNECTIONS`, `DEFAULT_PORT`

## Best Practices

### Module Independence
- Setiap module self-contained
- Tidak ada cross-module dependencies
- Share via common library jika perlu

### Middleware Locality
- Middleware lokal per module
- Tidak ada middleware global untuk domain logic
- Validasi & sanitasi di middleware module

### Service Layer
- Business logic hanya di service
- Validasi dua arah (handler ↔ DB)
- Error handling & transformation

### Repository Layer
- Pure database access
- No business logic
- Return Result types

### Error Handling
- Use AppError enum
- Transform errors di service layer
- Meaningful error messages

### Testing
- Unit tests untuk service logic
- Integration tests untuk handler + service + repository
- Mock repository untuk unit tests

## Dependencies

### Core
- `actix-web` - Web framework
- `tokio` - Async runtime
- `serde` - Serialization

### Database
- `sqlx` - PostgreSQL (compile-time checked queries)
- `mongodb` - MongoDB driver
- `redis` - Redis client

### Validation
- `validator` - Request validation

### Logging
- `tracing` - Structured logging
- `tracing-subscriber` - Log formatting
- `tracing-actix-web` - Actix integration

### Configuration
- `config` - Configuration management
- `dotenv` - Environment variables

### Utilities
- `chrono` - Date/time handling
- `uuid` - UUID generation
- `anyhow` - Error handling
- `thiserror` - Error derive macros

### Scheduler
- `tokio-cron-scheduler` - Cron job scheduling

## Environment Variables

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

## Build Artifacts

```
target/
├── debug/          # Development builds
│   ├── server
│   ├── worker
│   ├── scheduler
│   ├── migrator
│   └── seeder
│
└── release/        # Production builds (optimized)
    ├── server
    ├── worker
    ├── scheduler
    ├── migrator
    └── seeder
```

## Git Workflow

### Ignored Files
- `target/` - Build artifacts
- `.env` - Environment variables (sensitive)
- `Cargo.lock` - For libraries (keep for binaries)

### Tracked Files
- `.env.example` - Environment template
- `migrations/` - Database migrations
- `src/` - Source code
- `config/` - Configuration templates
