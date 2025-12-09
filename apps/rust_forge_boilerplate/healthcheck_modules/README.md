# Health Check Module

Simple health check module for monitoring service health and readiness.

## Endpoints

### 1. Health Check

**GET** `/api/health`

Simple health check that always returns OK. Useful for load balancer health checks.

**Response:**

```json
{
  "status": "ok",
  "service": "rust_forge_boilerplate",
  "version": "0.1.0"
}
```

### 2. Readiness Check

**GET** `/api/ready`

Comprehensive readiness check that verifies all dependencies (PostgreSQL, Redis, MongoDB).

**Response (Ready):**

```json
{
  "ready": true,
  "database": true,
  "redis": true,
  "mongodb": true
}
```

**Response (Not Ready):**

```json
{
  "ready": false,
  "database": true,
  "redis": false,
  "mongodb": true
}
```

Status code: `503 Service Unavailable` if not ready.

## Usage

This module is already registered in `cmd/server/main.rs` and ready to use.

```bash
# Test health
curl http://localhost:8080/api/health

# Test readiness
curl http://localhost:8080/api/ready
```

## Files

- **dto.rs** - Response data structures
- **handler.rs** - HTTP request handlers
- **mod.rs** - Module exports & route configuration

## Why This Module?

This module is useful for:

- ✅ Load balancer health checks
- ✅ Kubernetes liveness & readiness probes
- ✅ Monitoring & alerting
- ✅ Debugging dependency issues
- ✅ Production-ready boilerplate

**No need to delete** - this module will remain useful in production!
