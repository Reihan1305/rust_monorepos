# Rust Forge Boilerplate Architecture

## Core Principles

### 1. Local Middleware Per Module
Each module has its own `middleware.rs` for:
- Request body/header validation
- Input sanitization
- Type conversion
- Data pre-processing

**NO global middleware** - all validation and sanitization is local.

### 2. Layer Separation

```
Request → Routes → DTO/Middleware → Handler → Service → Repository → Database
                                      ↓           ↓
                                  Business     Data Access
                                   Logic       & Transform
```

**Key Concept:**
- **Handler** = Where business logic lives (orchestration, decisions, workflows)
- **Service** = Data access layer (fetch, validate, transform data for handler)

#### Routes
- Map endpoints to handlers
- Configured in each module's `mod.rs`

#### DTO & Middleware (LOCAL)
- **DTO**: Data types for request/response
- **Middleware**: Local validation & sanitization
- Each module has its own middleware
- No middleware sharing between modules

#### Handler
- **Business logic orchestration**
- Calls middleware for input validation
- Calls service to fetch/manipulate data
- Implements business rules and workflows
- Returns HTTP response
- Error handling and response formatting

#### Service
- **Data access layer**
- Fetches data from database via repository
- Data validation (format, constraints)
- Data transformation (DB model → DTO)
- Sanitization of data
- Makes data easy to read for handler
- Error handling from repository/database

#### Repository (Optional)
- Database access
- Query builder
- Used when module has multiple database options

#### Database
- Bottom layer
- Raw data & constraints

### Example Flow

**Creating a User:**

```rust
// Handler - Business Logic
pub async fn create_user(
    payload: web::Json<CreateUserDto>,
    pool: web::Data<PgPool>,
) -> AppResult<HttpResponse> {
    // 1. Validate input via middleware
    let validated_dto = middleware::validate_create_user(payload).await?;
    
    // 2. Business logic: Check if user can be created
    if validated_dto.age < 18 {
        return Err(AppError::BadRequest("Must be 18+".into()));
    }
    
    // 3. Call service to get/save data
    let user = service::create_user(&pool, validated_dto).await?;
    
    // 4. Business decision: Send welcome email
    email_service::send_welcome(&user.email).await?;
    
    // 5. Return response
    Ok(HttpResponse::Created().json(user))
}

// Service - Data Access
pub async fn create_user(
    pool: &PgPool,
    dto: CreateUserDto,
) -> AppResult<UserResponseDto> {
    // 1. Check if email exists (data validation)
    if repository::email_exists(pool, &dto.email).await? {
        return Err(AppError::BadRequest("Email exists".into()));
    }
    
    // 2. Save to database
    let user = repository::create(pool, dto).await?;
    
    // 3. Transform DB model to DTO (make it easy for handler)
    Ok(UserResponseDto {
        id: user.id,
        name: user.name,
        email: user.email,
        created_at: user.created_at,
    })
}
```

**Summary:**
- **Handler** decides WHAT to do (business rules, workflows)
- **Service** handles HOW to get/save data (database operations, transformations)

### 3. Common Library

```
common/
├── config.rs          # Application configuration
├── error.rs           # Error types & handling
├── infrastructure/    # Database, Redis, MongoDB connections
└── utils/             # General utilities (optional)
```

**NO domain logic** allowed in common.

## Entry Points

### 1. Server (HTTP)
```bash
cargo run --bin server
```
- Actix-web HTTP server
- Serve REST API
- Port default: 8080

### 2. Worker (Background Jobs)
```bash
cargo run --bin worker
```
- Process background jobs
- Consume from Redis queue
- Async job processing

### 3. Scheduler (Cron Tasks)
```bash
cargo run --bin scheduler
```
- Scheduled tasks with cron syntax
- Periodic jobs
- Maintenance tasks

### 4. Migrator (Database Migration)
```bash
cargo run --bin migrator
```
- Run database migrations
- Create/update schema
- SQLx migrations

### 5. Seeder (Database Seeder)
```bash
cargo run --bin seeder
```
- Seed initial data
- Test data generation
- Development data

## Module Structure Template

```rust
{module_name}/
├── dto.rs          # Data Transfer Objects
├── middleware.rs   # LOCAL validation & sanitization
├── handler.rs      # HTTP handlers
├── service.rs      # Data access layer
├── repository.rs   # Database access (optional)
└── mod.rs          # Module exports & route config
```

### dto.rs
```rust
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDto {
    #[validate(length(min = 3))]
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseDto {
    pub id: uuid::Uuid,
    pub name: String,
}
```

### middleware.rs
```rust
use crate::common::error::{AppError, AppResult};
use actix_web::web;
use validator::Validate;

pub async fn validate_create(
    payload: web::Json<CreateDto>,
) -> AppResult<CreateDto> {
    payload.validate().map_err(|e| {
        AppError::ValidationError(format!("Validation failed: {}", e))
    })?;

    // Sanitization
    let sanitized = CreateDto {
        name: payload.name.trim().to_string(),
    };

    Ok(sanitized)
}
```

### handler.rs
```rust
use crate::common::error::AppResult;
use actix_web::{web, HttpResponse};

pub async fn create(
    payload: web::Json<CreateDto>,
    pool: web::Data<PgPool>,
) -> AppResult<HttpResponse> {
    let validated = middleware::validate_create(payload).await?;
    let result = service::create(&pool, validated).await?;
    Ok(HttpResponse::Created().json(result))
}
```

### service.rs
```rust
use crate::common::error::{AppError, AppResult};

pub async fn create(
    pool: &PgPool,
    dto: CreateDto,
) -> AppResult<ResponseDto> {
    // Business validation
    if repository::exists_by_name(pool, &dto.name).await? {
        return Err(AppError::BadRequest("Name already exists".into()));
    }

    // Create
    let entity = repository::create(pool, dto).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Transform to response
    Ok(ResponseDto {
        id: entity.id,
        name: entity.name,
    })
}
```

### repository.rs
```rust
use sqlx::PgPool;

pub async fn create(
    pool: &PgPool,
    dto: CreateDto,
) -> Result<Entity, sqlx::Error> {
    sqlx::query_as::<_, Entity>(
        "INSERT INTO table (id, name) VALUES ($1, $2) RETURNING *"
    )
    .bind(uuid::Uuid::new_v4())
    .bind(dto.name)
    .fetch_one(pool)
    .await
}
```

### mod.rs
```rust
pub mod dto;
pub mod handler;
pub mod middleware;
pub mod repository;
pub mod service;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/resource")
            .route("", web::post().to(handler::create))
            .route("/{id}", web::get().to(handler::get))
    );
}
```

## Dependency Injection

Actix-web uses `web::Data` for DI:

```rust
HttpServer::new(move || {
    App::new()
        .app_data(web::Data::new(db_pool.clone()))
        .app_data(web::Data::new(redis_conn.clone()))
        .configure(module::configure_routes)
})
```

Handler receives dependencies via parameters:

```rust
pub async fn handler(
    pool: web::Data<PgPool>,
    redis: web::Data<ConnectionManager>,
) -> AppResult<HttpResponse> {
    // Use pool and redis
}
```

## Error Handling

All errors use `AppError` enum:

```rust
pub enum AppError {
    InternalError(String),
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    ValidationError(String),
    DatabaseError(String),
}
```

Service layer is responsible for:
- Catch database errors
- Transform to AppError
- Provide meaningful error messages

## Best Practices

### ✅ DO
- Local middleware per module
- Business logic in handler layer
- Data access and transformation in service layer
- Service validates and prepares data for handler
- Dependency injection via Actix Data
- Async/await for I/O operations

### ❌ DON'T
- Global middleware for domain logic
- Business logic in service layer (service is for data access)
- Direct database access from handler
- Domain logic in common library
- Blocking operations in async context
- Share middleware between modules

## Testing Strategy

### Unit Tests
- Test service logic
- Mock repository layer
- Test business rules

### Integration Tests
- Test handler + service + repository
- Use test database
- Test full request flow

### Example
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        let pool = create_test_pool().await;
        let dto = CreateUserDto {
            name: "Test".into(),
            email: "test@example.com".into(),
        };
        
        let result = service::create_user(&pool, dto).await;
        assert!(result.is_ok());
    }
}
```

## Configuration

### Environment Variables
```bash
APP__SERVER__HOST=127.0.0.1
APP__SERVER__PORT=8080
APP__DATABASE__URL=postgresql://...
APP__REDIS__URL=redis://...
```

### Config File
```toml
[server]
host = "127.0.0.1"
port = 8080

[database]
url = "postgresql://..."
max_connections = 10
```

Priority: Environment Variables > Config File > Defaults

## Database Migrations

### Create Migration
```bash
sqlx migrate add create_users_table
```

### Run Migrations
```bash
cargo run --bin migrator
```

### Migration File
```sql
-- migrations/20231201000000_create_users_table.sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
```

## Monitoring & Logging

### Tracing
```rust
tracing::info!("Server started");
tracing::error!("Database error: {}", e);
tracing::debug!("Processing request");
```

### Log Levels
- `RUST_LOG=debug` - Development
- `RUST_LOG=info` - Production
- `RUST_LOG=error` - Critical only

## Scalability

### Horizontal Scaling
- Stateless server design
- Session in Redis
- Database connection pooling

### Vertical Scaling
- Adjust `max_connections` in config
- Tune Actix worker threads
- Optimize database queries

## Security

### Input Validation
- All input validated in middleware
- Sanitization to prevent injection
- Type-safe with Rust

### Database
- Prepared statements (SQLx)
- Connection pooling
- No raw SQL injection

### Authentication
- Implement in separate module
- Middleware for auth check
- JWT/Session based
