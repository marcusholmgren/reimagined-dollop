# Rocket web server

This is a simple web server written in Rust. It is based on the [Rocket](https://rocket.rs/) web framework.

## Usage

To run the server, execute the following command:

```sh
cargo run
```

The server will start on `http://localhost:8000`.

## Endpoints

The server has the following endpoints:

- `/`: Returns a simple HTML page.
- `/api/hello/<name>`: Returns a JSON response with a greeting message.

### cURL examples

```sh
curl http://localhost:8000
```

```sh
curl http://localhost:8000/api/hello/John
```
