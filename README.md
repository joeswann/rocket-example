# Rust Web Project

This is a simple web server project built with Rust and the Rocket framework. It serves a "Hello, world!" message at the root URL.

## Project Structure

```
.
├── Cargo.toml
├── Cargo.lock
├── Makefile
├── rust-rocket.service
├── .gitignore
└── src
    └── main.rs
```

- `Cargo.toml`: Rust package manifest file
- `Cargo.lock`: Lock file for dependencies
- `Makefile`: Contains build and run commands
- `rust-rocket.service`: Systemd service file for deploying the server
- `.gitignore`: Specifies intentionally untracked files to ignore
- `src/main.rs`: Main application code

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Setup

1. Clone the repository:
   ```
   git clone <repository-url>
   cd rust-web
   ```

2. Build the project:
   ```
   make build
   ```

## Running the Server

To run the server locally:

```
make run
```

The server will start and be available at `http://localhost:8000`.

## Deployment

This project includes a systemd service file for easy deployment on Linux systems.

1. Build the release version:
   ```
   make build
   ```

2. Install the systemd service:
   ```
   make install
   ```

   Note: You may need to modify the paths in `rust-rocket.service` to match your deployment environment.

3. Start the service:
   ```
   sudo systemctl start rust-rocket
   ```

4. Enable the service to start on boot:
   ```
   sudo systemctl enable rust-rocket
   ```

## API Endpoints

- `GET /`: Returns "Hello, world! Welcome to Rocket."

## Development

To add new routes or modify the existing one, edit the `src/main.rs` file. After making changes, rebuild and restart the server.

## License

This project is licensed under the MIT License.
