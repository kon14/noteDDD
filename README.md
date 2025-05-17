<div align="center">
<br>
<a href="https://github.com/kon14/noteDDD" target="_blank">
    <h1>noteDDD 🗺️</h1>
</a>
A template for <strong>Rust</strong>-based services utilizing <strong>Domain-Driven Design</strong> and <strong>Clean Architecture</strong>.
</div>

<hr />

This template serves as a production-ready foundation for building scalable **Rust** 🦀 microservices, while maintaining **Clean Architecture (CA)** principles and strict adherence to **Domain-Driven Design (DDD)** patterns.<br />
It's meant to function as a comprehensive demonstration of how to structure a clean, maintainable codebase with strict layer separation and proper dependency management.

---

## Running via Docker Compose 💻 <a name="run-compose"></a>

``` bash
# Start the service and PostgreSQL
docker compose up --build

# Navigate to Swagger UI (on Linux)
xdg-open "http://localhost:4000/swagger/index.html"
```

*Note: Make sure you bring up the full Swagger UI path to avoid 404s caused by path normalization redirects.*

---

## Architecture Overview 🏗️

The project is organized into **4 distinct layers** + **2 supporting crates**, each with clear responsibilities and enforced boundaries:

### **Domain Layer** (`dmn/`)
The core business logic layer containing:
- **Entities**: Core business objects with identity
- **Value Objects**: Immutable types representing domain concepts
- **Repository Traits**: Abstract contracts for data persistence
- **Domain Validation**: Business rule enforcement at the entity level

### **Application Layer** (`app/`)
Orchestrates business workflows without implementation details:
- **Use Cases**: Application-specific business logic
- **Application Services**: Cross-cutting concerns (authentication, authorization)
- **Repository Traits**: Abstract data access contracts
- **Application DTOs**: Data transfer objects for interlayer communication

### **Infrastructure Layer** (`infra/`)
Concrete implementations of abstract contracts:
- **Repository Implementations**: **PostgreSQL**-based data persistence
- **External Service Adapters**: JWT handling, password hashing
- **Database Queries**: Raw SQL operations using **SQLx**
- **Infrastructure DTOs**: Database-specific data models

### **Presentation Layer** (`pres/`)
HTTP API and external interface:
- **HTTP Handlers**: Axum-based REST endpoints
- **DTOs**: API request/response models
- **Authentication Extractors**: JWT token validation
- **OpenAPI Documentation**: Auto-generated via **utoipa**

### **Common** (`common/`)
Shared utilities across all layers:
- **Error Types**: Standardized error handling with sensitive information obfuscation
- **Transaction Abstractions**: Infrastructure-agnostic transaction management (allows for CA-compliant use cases)
- **Cross-cutting Utilities**: Shared types and helper functions

### **Bootstrap** (`main/`)
Application entry point and dependency injection:
- **Dependency Wiring**: Service registration and configuration
- **Application Startup**: Server initialization and middleware setup

## Clean Architecture Dependency Flow 🔄

The layers follow strict dependency rules to maintain clean architecture:

- **Domain** depends on nothing (pure business logic)
- **Application** depends solely on <em>Domain</em>
- **Infrastructure** depends on <em>Domain</em> and <em>Application</em>
- **Presentation** depends on <em>Application</em> and <em>Domain</em>
- **Common** is dependency-free and accessible by all layers
- **Bootstrap** depends on all layers to wire everything together

---

## Key Features 🌟

### **Crate-Level Isolation**
Each layer is a separate Rust crate, enforcing architectural boundaries at compile-time.<br />
This prevents accidental violations of the dependency rule.

### **Sensitive Information Protection**
Implements a **two-layer error system**:
- **Public Errors**: Safe for client consumption
- **Private Errors**: Internal details preserved for internal debugging and logging
- Sensitive information is obfuscated before reaching the client

### **Transaction Management**
Application-layer controlled transactions without leaking infrastructure concerns:
- Domain-agnostic transaction abstractions
- Infrastructure-specific implementations
- Full CA compliance without leaking database concerns

### **Comprehensive Documentation**
- **OpenAPI/Swagger UI**: Auto-generated API documentation
- **Type-Safe DTOs**: Compile-time API contract validation
- **Security Annotations**: Clear authentication requirements per endpoint

### **Production-Ready Features**
- **JWT Authentication**: Access/refresh token pairs with secure rotation
- **Password Security**: **BCrypt** hashing with configurable complexity
- **Database Migrations**: Version-controlled schema management
- **Docker Support**: Containerized deployment with multi-stage builds
- **Structured Logging**: Configurable log levels with sensitive data filtering

---

## Local Development 👨🏻‍🔬 <a name="local-dev"></a>

The following section assumes your environment contains an installation of the [Rust development toolchain](https://www.rust-lang.org/tools/install).

``` bash
# Prepare Git Hooks (Optional)
lefthook install

# Install the SQLX CLI
cargo install sqlx-cli --no-default-features --features postgres
```

``` bash
# Apply DB Migrations
DATABASE_URL="postgres://noteddd:pass@localhost:5432/noteddd" sqlx migrate run

# Build noteDDD
cargo build

# Run noteDDD
DATABASE_URL="postgres://noteddd:pass@localhost:5432/noteddd" \
API_BASE_URL="http://localhost:4000" \
AUTH_JWT_SECRET="7h3 c4k3 15 4 l13" \
cargo run

# Navigate to Swagger UI (on Linux)
xdg-open "http://localhost:4000/swagger/index.html"
```

---

## Environment Variables 📃 <a name="env-vars"></a>

|              Variable              | Description                                                                                                                                                                                                          | Required |         Default          |            Example             |
|:----------------------------------:|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|:--------:|:------------------------:|:------------------------------:|
|           `DATABASE_URL`           | The connection string URL for your PostgreSQL database.                                                                                                                                                              |  `True`  |            —             | `postgres://localhost:5432/db` |
|             `API_PORT`             | The port to be used by the HTTP server.                                                                                                                                                                              | `False`  |          `4000`          |             `8080`             |
|           `API_BASE_URL`           | A public URL pointing to the backend API's root path.                                                                                                                                                                |  `True`  |            —             |   `https://foo.bar.baz/api`    |
|             `RUST_LOG`             | Specifies the desired logging level.<br />Refer to the [env_logger](https://docs.rs/env_logger/latest/env_logger/) documentation for details.                                                                        | `False`  |         `error`          |             `info`             |
|         `AUTH_JWT_SECRET`          | The secret to be used for JWT authentication token encoding/decoding.                                                                                                                                                |  `True`  |            —             |      `7h3 c4k3 15 4 l13`       |
| `AUTH_ACCESS_TOKEN_DURATION_SECS`  | Duration for authentication access token validity (in seconds).                                                                                                                                                      | `False`  |   `5 * 60` (5 minutes)   |             `300`              |
| `AUTH_REFRESH_TOKEN_DURATION_SECS` | Duration for authentication refresh token validity (in seconds).                                                                                                                                                     | `False`  |  `24 * 60 * 60` (1 day)  |            `86400`             |
