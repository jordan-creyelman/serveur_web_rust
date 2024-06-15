# serveur_web_rust

# Rust Web Server with Middleware

This is a simple web server built with Rust using the Actix Web framework. It demonstrates the use of middleware for logging and error handling.

## Prerequisites

Before running this project, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Getting Started

### 1. Clone the repository

```sh
git clone https://github.com/yourusername/rust-web-server.git
cd rust-web-server
2. Build the project
Use Cargo to build the project:

sh
Copier le code
cargo build
3. Run the server
Start the server with:

sh
Copier le code
cargo run
The server will start running on http://127.0.0.1:8088.

Project Structure
src/main.rs: The main entry point of the server.
index.html: The HTML file served at the root route (/).
about.html: The HTML file served at the /about route.
Endpoints
GET /: Serves the index.html file.
GET /about: Serves the about.html file.
GET /error: Demonstrates error handling with custom middleware.
Middleware
This project uses the Actix Web Logger middleware for logging requests.
