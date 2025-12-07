# Rust Web Starter

A production-ready Rust web application template with a clean, modular architecture following domain-driven design principles.

## Overview

This project provides a foundational template for developing web applications and RESTful APIs using Rust. It demonstrates best practices with a layered architecture (controller → service → repository) and includes essential features out of the box.

## Features

- **Actix-web** - High-performance async web framework built on Tokio
- **Actix-files** - Static file serving for assets (CSS, JavaScript, images)
- **Handlebars** - Server-side templating engine for dynamic HTML rendering
- **MongoDB** - NoSQL database integration with async driver
- **Redis** - In-memory data store with connection pooling and automatic reconnection
- **JSON API** - RESTful endpoints with JSON request/response handling
- **Logging** - Structured logging with `env_logger` and configurable log levels
- **Time** - Modern date and time handling with ISO 8601 support
- **Docker** - Multi-stage Docker build for optimized production images

## Architecture

The project follows a modular, layered architecture with a clear folder structure paradigm:

```
src/
├── lib.rs                      # Library root (public modules)
├── main.rs                     # Application entry point
├── home/                       # Home page module (renders HTML)
│   ├── mod.rs                 # Module exports
│   ├── dto.rs                 # View models for templates
│   └── home_render.rs         # Handlebars route handlers
├── shared/                     # Shared utilities and configurations
│   ├── config/
│   │   └── config.rs          # Server configuration, MongoDB, logging
│   └── dto/
│       └── response.rs        # HTTP response helpers
└── users/                      # Users domain module (REST API)
    ├── mod.rs                  # Module exports
    ├── dto.rs                  # Data transfer objects
    ├── users_controller.rs     # REST API handlers (JSON responses)
    ├── users_service.rs        # Business logic layer
    ├── users_repository.rs     # Data access layer
    └── users_model.rs          # Domain models
```

### Folder Structure Paradigm

The project follows a consistent naming convention to distinguish between different types of routes:

#### Naming Convention
- **`{path}_render.rs`** - Handlebars template rendering (returns `text/html`)
  - Example: `home_render.rs` serves HTML pages at `/`
  - Used for server-side rendered web pages

- **`{path}_controller.rs`** - REST API endpoints (returns `application/json`)
  - Example: `users_controller.rs` serves JSON API at `/users`
  - Used for RESTful web services

#### Module Structure
Each feature module follows this pattern:
- **Folder name** = URL path segment (e.g., `home/` → `/`, `users/` → `/users`)
- **File suffix** indicates response type (`_render` for HTML, `_controller` for JSON)
- **`dto.rs`** contains data transfer objects (request/response models)
- **`mod.rs`** exports public module members

#### Adding New Features

**To add a new HTML page** (e.g., `/about`):
```
src/about/
├── mod.rs
├── dto.rs                 # View models
└── about_render.rs        # GET /about → renders about.hbs
```

**To add a new REST API** (e.g., `/products`):
```
src/products/
├── mod.rs
├── dto.rs                      # Request/response DTOs
├── products_controller.rs      # REST endpoints
├── products_service.rs         # Business logic
├── products_repository.rs      # Database operations
└── products_model.rs           # Domain models
```

### Design Patterns

- **Layered Architecture**: Controller → Service → Repository pattern for separation of concerns
- **Domain Modules**: Features organized by domain (home, users, etc.) for scalability
- **Clear Separation**: Render modules for HTML, controller modules for JSON APIs
- **Dependency Injection**: MongoDB client, Redis, and Handlebars injected via Actix-web Data
- **Flexible SSR**: If you don't need server-side rendering, simply exclude Handlebars dependencies and render routes - the project works perfectly as a JSON API-only backend

## Getting Started

### Prerequisites

- Rust 2024 edition or later
- Cargo package manager
- MongoDB instance (local or remote)
- Redis instance (local or remote)

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
- `MONGODB_TIMEOUT_SECS` - MongoDB connection timeout in seconds (default: `10`)

#### Cache Configuration
- `REDIS_URI` - Redis connection string (default: `redis://localhost:6379`)
- `REDIS_TIMEOUT_SECS` - Redis connection timeout in seconds (default: `10`)

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
docker run -d -p 3000:3000 rust-web-starter

# Run with custom environment variables
docker run -d -p 3000:3000 \
  -e BIND_PORT=3000 \
  -e MONGODB_URI=mongodb://mongo:27017 \
  -e REDIS_URI=redis://redis:6379 \
  -e RUST_LOG=info \
  rust-web-starter

# Run with MongoDB and Redis using docker-compose
docker-compose up -d
```

#### Docker Compose

The project includes a `docker-compose.yml` that orchestrates all services:

```bash
# Start all services (app, MongoDB, Redis)
docker-compose up -d

# View logs
docker-compose logs -f

# Stop all services
docker-compose down

# Rebuild and restart
docker-compose up -d --build
```

Services included:
- **rust-web-starter** - The web application (port 3000)
- **mongodb** - MongoDB database (port 27017)
- **redis** - Redis cache (port 6379)

Data persistence:
- MongoDB data: `./data/mongodb`
- Redis data: `./data/redis`

The Dockerfile uses a multi-stage build:
- **Stage 1 (builder)**: Compiles the Rust application with layer caching for dependencies
- **Stage 2 (runtime)**: Minimal Debian slim image with only the binary and assets

## License

MIT

## Author

Alberto Ielpo