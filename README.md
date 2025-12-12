# NX Monorepo Blueprint

<a alt="Nx logo" href="https://nx.dev" target="_blank" rel="noreferrer"><img src="https://raw.githubusercontent.com/nrwl/nx/master/images/nx-logo.png" width="45"></a>

A monorepos blueprint powered by [Nx](https://nx.dev) and [@monodon/rust](https://github.com/nrwl/monodon) for efficient development and build management.

## 🚀 Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://rustup.rs/) (latest stable)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### Installation

```bash
# Install dependencies
npm install

# Verify Rust installation
rustc --version
cargo --version
```

## 📁 Project Structure

```
├── apps/                    # applications
│   └── rust_forge_boilerplate/  # application
├── libs/                    # Shared libraries
│   └── errors/             # Error handling library
├── tools/                   # Development tools
│   ├── create-rust-app.js  # App generator script
│   └── rust_app_template/  # Template for new apps
└── dist/                   # Build outputs
```

## 🛠️ Development Commands

### Building Projects

```bash
# Build all projects
npx nx run-many -t build

# Build specific project
npx nx build rust_forge_boilerplate
npx nx build errors

# Build with release optimizations
npx nx build rust_forge_boilerplate --configuration=production
```

### Testing

```bash
# Run all tests
npx nx run-many -t test

# Test specific project
npx nx test rust_forge_boilerplate
npx nx test errors

# Run tests with coverage
npx nx test rust_forge_boilerplate --configuration=production
```

### Linting

```bash
# Lint all projects
npx nx run-many -t lint

# Lint specific project
npx nx lint rust_forge_boilerplate
```

### Running Applications

```bash
# Run the main application
npx nx run rust_forge_boilerplate

# Run with release optimizations
npx nx run rust_forge_boilerplate --configuration=production
```

## 📦 Creating New Projects

### Generate a New Rust Library

```bash
# Create a new library
npx nx g @monodon/rust:lib my-new-lib

# Create library with specific features
npx nx g @monodon/rust:lib utils --directory=shared
```

### Generate a New Rust Application

```bash
# Use the custom generator script
npm run create:rust-app

# Or use Nx generator directly
npx nx g @monodon/rust:app my-new-app
```

## 🔧 Available Scripts

- `npm run create:rust-app` - Interactive script to create new Rust applications

## 📊 Project Visualization

```bash
# View project dependency graph
npx nx graph

# Show affected projects (useful for CI)
npx nx affected:graph
```

## 🏗️ Build Targets

Each Rust project supports these targets:

- **build** - Compile the project
- **test** - Run unit tests
- **lint** - Run clippy linting
- **run** - Execute the binary (for applications)

### Configuration Options

- **production** - Optimized release builds with `--release` flag

## 🚀 Deployment

Build outputs are generated in the `dist/target/` directory:

```bash
# Production builds
npx nx build rust_forge_boilerplate --configuration=production

# Outputs will be in: dist/target/rust_forge_boilerplate/release/
```

## 🔍 Troubleshooting

### Common Issues

1. **Duplicate project names**: Ensure each project has a unique name in its `project.json`
2. **Cargo.lock conflicts**: Run `cargo update` in the workspace root
3. **Build cache issues**: Clear with `npx nx reset`

### Useful Commands

```bash
# Clear Nx cache
npx nx reset

# Show project information
npx nx show project rust_forge_boilerplate

# List all projects
npx nx show projects
```
