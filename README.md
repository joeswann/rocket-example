# Rust Web Project

This is an asynchronous, multi-threaded web server project built with Rust and the Rocket framework. It's designed to handle a large number of concurrent connections efficiently.

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

The server will start and be available at `http://0.0.0.0:8000`.

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

- `GET /`: Returns a welcome message with a hit counter.
- `GET /delay/<ms>`: Returns a delayed response after the specified number of milliseconds.

## Asynchronous Processing

The server uses asynchronous processing to handle requests efficiently. This allows it to manage a large number of concurrent connections without blocking.

## Multi-threading

The server is configured to use a number of worker threads equal to twice the number of CPU cores. This allows it to efficiently handle multiple concurrent requests and potentially saturate the network connection.

## Performance Testing

To test the server's performance and ability to handle multiple concurrent requests, you can use tools like Apache Bench (ab) or wrk. For example:

```
ab -n 10000 -c 1000 http://localhost:8000/
```

This will send 10000 requests with 1000 concurrent connections to the root endpoint.

For testing with delay:

```
ab -n 1000 -c 100 http://localhost:8000/delay/50
```

This will send requests that each have a 50ms delay, allowing you to simulate slower operations.

## Development

To add new routes or modify the existing ones, edit the `src/main.rs` file. After making changes, rebuild and restart the server.

## License

This project is licensed under the MIT License.
