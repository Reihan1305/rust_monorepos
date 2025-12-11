# Rust App Generator

This tool creates new Rust applications using a template stored in `tools/templates/rust_app_template/`.

## Usage

```bash
npm run create:rust-app <app_name>
```

## Rules

- App names must start with a letter
- Only letters, numbers, and underscores are allowed
- No hyphens allowed (use underscores instead)
- Maximum 50 characters

## Examples

```bash
# ✅ Good names
npm run create:rust-app my_awesome_app
npm run create:rust-app user_service
npm run create:rust-app api_gateway

# ❌ Bad names (will be rejected)
npm run create:rust-app my-bad-app    # Contains hyphens
npm run create:rust-app 123invalid    # Starts with number
npm run create:rust-app my@app        # Contains special characters
```

## What it creates

The generator will:

1. Copy all files from the template directory `tools/templates/rust_app_template/`
2. Replace the package name in all files
3. Add the new app to the workspace `Cargo.toml`
4. Create a proper Nx project configuration

## Template Location

The template is stored in `tools/templates/rust_app_template/` and contains all the boilerplate code for a new Rust application. This keeps the template separate from your actual applications.

## Generated structure

```
apps/your_app_name/
├── cmd/                    # Binary executables
│   ├── server/            # Web server
│   ├── worker/            # Background worker
│   ├── scheduler/         # Cron scheduler
│   ├── migrator/          # Database migrations
│   └── seeder/            # Database seeding
├── common/                # Shared modules
├── healthcheck_modules/   # Health check endpoints
├── deployment/            # Docker files
├── migrations/            # SQL migrations
├── Cargo.toml            # Rust package config
├── project.json          # Nx project config
└── .env.example          # Environment template
```

## Available commands

After creating an app, you can use these Nx commands:

```bash
nx build your_app_name     # Build the app
nx test your_app_name      # Run tests
nx lint your_app_name      # Run linter
nx run your_app_name       # Run the app
```