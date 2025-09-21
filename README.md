# Full Stack Rust

This is a full-stack web application built with Rust.

## Project Structure

- `src/`: Contains the Rust source code.
  - `main.rs`: The entry point of the application.
  - `routes.rs`: Defines the applications routes.
  - `handlers/`: Contains the request handlers for different routes.
  - `models/`: Contains data models and templates.
- `static/`: Contains static assets like CSS, JavaScript, and images.
- `templates/`: Contains the HTML templates for the web pages.
- `Cargo.toml`: The package manifest for the Rust project.
- `Makefile`: Contains commands for building and running the project.

## Getting Started

### Prerequisites

- [Rust](https.rust-lang.org/)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/full-stack-rust.git
   ```
2. Navigate to the project directory:
   ```bash
   cd full-stack-rust
   ```
3. Build the project:
   ```bash
   make build
   ```

### Running the Application

To run the application, use the following command:

```bash
make run
```

The application will be available at `http://127.0.0.1:8000`.

## Routes

- `/`: Home page.
- `/log-in`: Log in page.
- `/sing-up`: Sign up page.
- `/creates`: Page to create a new to-do item.
- `/todos`: Page to view all to-do items.

## Dependencies

- [axum](https://github.com/tokio-rs/axum): A web application framework that focuses on ergonomics and modularity.
- [tokio](https://tokio.rs/): An asynchronous runtime for Rust.
- [askama](https://github.com/djc/askama): A type-safe, compiled Jinja-like template engine for Rust.
- [tower-http](https://github.com/tower-rs/tower-http): A library of HTTP-related middleware and utilities for Tower.
- [tracing](https://github.com/tokio-rs/tracing): A framework for instrumenting Rust programs to collect structured, event-based diagnostic information.

