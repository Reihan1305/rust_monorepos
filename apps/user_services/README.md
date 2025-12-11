# User Services

A Rust-based microservice for user management with support for PostgreSQL, Redis, and MongoDB.

## Configuration

This application uses environment variables for configuration. Copy `.env.example` to `.env` and modify the values as needed.

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SERVER_HOST` | Server bind address | `127.0.0.1` |
| `SERVER_PORT` | Server port | `8080` |
| `DATABASE_URL` | PostgreSQL connection URL | Required |
| `DATABASE_MAX_CONNECTIONS` | Maximum database connections | `10` |
| `REDIS_URL` | Redis connection URL | `redis://localhost:6379` |
| `MONGODB_URL` | MongoDB connection URL | `mongodb://localhost:27017` |
| `MONGODB_DATABASE` | MongoDB database name | `user_services_db` |
| `RUST_LOG` | Logging level | `info` |

## Running the Application

### Prerequisites

1. Copy the environment file:
   ```bash
   cp .env.example .env
   ```

2. Update the `.env` file with your configuration values.

### Available Commands

- **Server**: `cargo run --bin server`
- **Worker**: `cargo run --bin worker`
- **Scheduler**: `cargo run --bin scheduler`
- **Migrator**: `cargo run --bin migrator`
- **Seeder**: `cargo run --bin seeder`

### Development

```bash
# Run migrations
cargo run --bin migrator

# Seed the database
cargo run --bin seeder

# Start the server
cargo run --bin server
```

### Docker Deployment

#### Development Environment

```bash
# Start all services with docker-compose
cd deployment
docker compose up -d

# View logs
docker compose logs -f

# Stop services
docker compose down
```

#### Production Environment

1. Copy the production environment file:
   ```bash
   cd deployment
   cp .env.prod.example .env.prod
   ```

2. Update `.env.prod` with your production values

3. Deploy:
   ```bash
   docker compose -f docker-compose.prod.yml up -d
   ```

## Architecture

The application follows a modular architecture:

- `cmd/` - Application entry points (binaries)
- `common/` - Shared utilities and infrastructure
- `user_modules/` - User-related business logic
- `healthcheck_modules/` - Health check endpoints
- `migrations/` - Database migrations