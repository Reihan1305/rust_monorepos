# 🚀 Deployment Guide

Panduan deployment untuk Rust Forge Boilerplate menggunakan Docker.

## 📁 Structure

```
deployment/
├── Dockerfile.server          # Server HTTP API
├── Dockerfile.worker          # Background worker
├── Dockerfile.scheduler       # Task scheduler
├── Dockerfile.migrator        # Database migrator
├── Dockerfile.seeder          # Database seeder
├── docker-compose.yml         # Development setup
├── docker-compose.prod.yml    # Production setup
├── .env.example               # Environment template
├── .env.prod.example          # Production env template
└── README.md                  # This file
```

## 🛠️ Development Deployment

### Quick Start

```bash
cd deployment
docker-compose up -d
```

Ini akan menjalankan:
- PostgreSQL (port 5432)
- Redis (port 6379)
- MongoDB (port 27017)
- Migrator (run once)
- Server (port 8080)
- Worker
- Scheduler

### Check Status

```bash
# View all services
docker-compose ps

# View logs
docker-compose logs -f

# View specific service logs
docker-compose logs -f server
docker-compose logs -f worker
docker-compose logs -f scheduler
```

### Stop Services

```bash
docker-compose down

# Stop and remove volumes
docker-compose down -v
```

## 🏭 Production Deployment

### Prerequisites

1. Create `.env.prod` file:

```bash
# Database
DB_NAME=rust_forge_db
DB_USER=your_user
DB_PASSWORD=your_secure_password

# Redis
REDIS_PASSWORD=your_redis_password

# MongoDB
MONGO_USER=your_mongo_user
MONGO_PASSWORD=your_mongo_password
MONGO_DB=rust_forge_db

# Registry (for production images)
REGISTRY=your-registry.com
VERSION=1.0.0
```

2. Build and push images:

```bash
# Build all images
docker-compose -f docker-compose.prod.yml build

# Tag images
docker tag rust-forge-server:latest ${REGISTRY}/rust-forge-server:${VERSION}
docker tag rust-forge-worker:latest ${REGISTRY}/rust-forge-worker:${VERSION}
docker tag rust-forge-scheduler:latest ${REGISTRY}/rust-forge-scheduler:${VERSION}
docker tag rust-forge-migrator:latest ${REGISTRY}/rust-forge-migrator:${VERSION}

# Push to registry
docker push ${REGISTRY}/rust-forge-server:${VERSION}
docker push ${REGISTRY}/rust-forge-worker:${VERSION}
docker push ${REGISTRY}/rust-forge-scheduler:${VERSION}
docker push ${REGISTRY}/rust-forge-migrator:${VERSION}
```

3. Deploy:

```bash
cd deployment
docker-compose -f docker-compose.prod.yml up -d
```

## 🔧 Individual Service Deployment

### Server Only

```bash
# Build (from project root)
docker build -f deployment/Dockerfile.server -t rust-forge-server .

# Run
docker run -d \
  --name rust-forge-server \
  -p 8080:8080 \
  --env-file deployment/.env \
  rust-forge-server
```

### Worker Only

```bash
# Build (from project root)
docker build -f deployment/Dockerfile.worker -t rust-forge-worker .

# Run
docker run -d \
  --name rust-forge-worker \
  --env-file deployment/.env \
  rust-forge-worker
```

### Scheduler Only

```bash
# Build (from project root)
docker build -f deployment/Dockerfile.scheduler -t rust-forge-scheduler .

# Run
docker run -d \
  --name rust-forge-scheduler \
  --env-file deployment/.env \
  rust-forge-scheduler
```

### Run Migrations

```bash
# Build (from project root)
docker build -f deployment/Dockerfile.migrator -t rust-forge-migrator .

# Run (one-time)
docker run --rm \
  --env-file deployment/.env \
  rust-forge-migrator
```

### Run Seeder

```bash
# Build (from project root)
docker build -f deployment/Dockerfile.seeder -t rust-forge-seeder .

# Run (one-time)
docker run --rm \
  --env-file deployment/.env \
  rust-forge-seeder
```

## 📊 Monitoring

### Health Checks

```bash
# Server health
curl http://localhost:8080/api/health

# Expected response
{
  "status": "ok",
  "service": "rust_forge_boilerplate",
  "version": "0.1.0"
}

# Readiness check (checks all dependencies)
curl http://localhost:8080/api/ready

# Expected response
{
  "ready": true,
  "database": true,
  "redis": true,
  "mongodb": true
}
```

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f server
docker-compose logs -f worker
docker-compose logs -f scheduler

# Last 100 lines
docker-compose logs --tail=100 server
```

### Resource Usage

```bash
# View resource usage
docker stats

# Specific container
docker stats rust-forge-server
```

## 🔄 Updates & Rollback

### Update Services

```bash
# Pull latest images
docker-compose pull

# Restart services
docker-compose up -d

# Or rebuild and restart
docker-compose up -d --build
```

### Rollback

```bash
# Stop current version
docker-compose down

# Deploy previous version
docker-compose up -d
```

## 🐛 Troubleshooting

### Server won't start

```bash
# Check logs
docker-compose logs server

# Check if port is available
netstat -tulpn | grep 8080

# Restart service
docker-compose restart server
```

### Database connection error

```bash
# Check if PostgreSQL is running
docker-compose ps postgres

# Check PostgreSQL logs
docker-compose logs postgres

# Test connection
docker-compose exec postgres psql -U user -d rust_forge_db
```

### Worker not processing jobs

```bash
# Check worker logs
docker-compose logs worker

# Check Redis connection
docker-compose exec redis redis-cli ping

# Restart worker
docker-compose restart worker
```

### Migration fails

```bash
# Check migrator logs
docker-compose logs migrator

# Run migration manually
docker-compose run --rm migrator
```

## 🔐 Security Best Practices

### Production Checklist

- [ ] Use strong passwords for all services
- [ ] Don't expose database ports publicly
- [ ] Use secrets management (Docker secrets, Vault)
- [ ] Enable SSL/TLS for all connections
- [ ] Run containers as non-root user (already configured)
- [ ] Use private Docker registry
- [ ] Enable Docker content trust
- [ ] Regular security updates
- [ ] Implement rate limiting
- [ ] Use firewall rules

### Environment Variables

Never commit `.env` files with real credentials. Use `.env.example` as template:

```bash
# Copy example
cp .env.example .env

# Edit with real values
nano .env
```

## 📈 Scaling

### Horizontal Scaling

Scale specific services:

```bash
# Scale server to 3 instances
docker-compose up -d --scale server=3

# Scale worker to 5 instances
docker-compose up -d --scale worker=5
```

**Note**: Scheduler should only run 1 instance to avoid duplicate jobs.

### Resource Limits

Edit `docker-compose.prod.yml` to adjust resources:

```yaml
services:
  server:
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 1G
        reservations:
          cpus: '1'
          memory: 512M
```

## 🔄 Backup & Restore

### Backup Database

```bash
# PostgreSQL backup
docker-compose exec postgres pg_dump -U user rust_forge_db > backup.sql

# MongoDB backup
docker-compose exec mongodb mongodump --out=/backup
```

### Restore Database

```bash
# PostgreSQL restore
docker-compose exec -T postgres psql -U user rust_forge_db < backup.sql

# MongoDB restore
docker-compose exec mongodb mongorestore /backup
```

## 📝 Useful Commands

```bash
# View all containers
docker-compose ps

# Stop all services
docker-compose stop

# Start all services
docker-compose start

# Restart specific service
docker-compose restart server

# Remove all containers
docker-compose down

# Remove containers and volumes
docker-compose down -v

# View resource usage
docker-compose top

# Execute command in container
docker-compose exec server sh

# View container details
docker inspect rust-forge-server
```

## 🌐 Nginx Reverse Proxy (Optional)

Create `nginx.conf`:

```nginx
upstream rust_forge_backend {
    least_conn;
    server server:8080;
}

server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://rust_forge_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /health {
        proxy_pass http://rust_forge_backend/health;
        access_log off;
    }
}
```

Add to `docker-compose.yml`:

```yaml
nginx:
  image: nginx:alpine
  ports:
    - "80:80"
  volumes:
    - ./nginx.conf:/etc/nginx/conf.d/default.conf:ro
  depends_on:
    - server
  networks:
    - rust_forge_network
```

## 📚 Additional Resources

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [Actix-web Deployment](https://actix.rs/docs/server/)
- [PostgreSQL Docker](https://hub.docker.com/_/postgres)
- [Redis Docker](https://hub.docker.com/_/redis)
- [MongoDB Docker](https://hub.docker.com/_/mongo)
