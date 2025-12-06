# Rust Web Starter

A production-ready Rust web application template with a clean, modular architecture following domain-driven design principles.

## Overview

This project provides a foundational template for developing web applications and RESTful APIs using Rust. It demonstrates best practices with a layered architecture (controller → service → repository) and includes essential features out of the box.

## Features

- **Actix-web** - High-performance async web framework built on Tokio
- **Actix-files** - Static file serving for assets (CSS, JavaScript, images)
- **Handlebars** - Server-side templating engine for dynamic HTML rendering
- **MongoDB** - NoSQL database integration with async driver
- **JSON API** - RESTful endpoints with JSON request/response handling
- **Logging** - Structured logging with `env_logger` and configurable log levels
- **Time** - Modern date and time handling with ISO 8601 support
- **Docker** - Multi-stage Docker build for optimized production images

## Architecture

The project follows a modular, layered architecture:

```
src/
├── lib.rs                      # Library root (public modules)
├── main.rs                     # Application entry point
├── shared/                     # Shared utilities and configurations
│   ├── config/
│   │   └── config.rs          # Server configuration, MongoDB, logging
│   └── dto/
│       └── response.rs        # HTTP response helpers
└── users/                     # Users domain module
    ├── dto.rs                 # Data transfer objects
    ├── users_controller.rs   # HTTP handlers and routes
    ├── users_service.rs      # Business logic layer
    ├── users_repository.rs   # Data access layer
    └── users_model.rs        # Domain models
```

### Design Patterns

- **Layered Architecture**: Controller → Service → Repository pattern for separation of concerns
- **Domain Modules**: Features organized by domain (users, etc.) for scalability
- **Dependency Injection**: MongoDB client and Handlebars injected via Actix-web Data

## Getting Started

### Prerequisites

- Rust 2024 edition or later
- Cargo package manager
- MongoDB instance (local or remote)

### Installation

```bash
cargo build
```

### Running the Application

```bash
cargo run
```

The server will start on `http://0.0.0.0:3000` by default.

### Configuration

Environment variables for customization:

#### Server Configuration
- `BIND_ADDR` - Server bind address (default: `0.0.0.0`)
- `BIND_PORT` - Server port (default: `3000`)
- `RUST_LOG` - Log level: `error`, `warn`, `info`, `debug`, `trace` (default: `debug`)

#### Database Configuration
- `MONGODB_URI` - MongoDB connection string (default: `mongodb://localhost:27017`)

#### Path Configuration
- `TEMPLATES_DIR` - Path to Handlebars templates (default: `./templates`)
- `ASSETS_DIR` - Path to static assets (default: `./assets`)

## Development Commands

### Build & Run

```bash
# Build the project
cargo build

# Run the application
cargo run

# Watch mode (requires cargo-watch)
cargo watch -x "run"

# Build for release
cargo build --release
```

### Docker

#### Build Docker Image

```bash
docker build -t rust-web-starter .
```

#### Run Docker Container

```bash
# Run with default settings
docker run -d -p 8080:8080 rust-web-starter

# Run with custom environment variables
docker run -d -p 8080:8080 \
  -e BIND_PORT=8080 \
  -e MONGODB_URI=mongodb://mongo:27017 \
  -e RUST_LOG=info \
  rust-web-starter

# Run with MongoDB using docker-compose
# (Create a docker-compose.yml for this)
docker-compose up -d
```

The Dockerfile uses a multi-stage build:
- **Stage 1 (builder)**: Compiles the Rust application with layer caching for dependencies
- **Stage 2 (runtime)**: Minimal Debian slim image with only the binary and assets

## License

MIT

## Author

Alberto Ielpo