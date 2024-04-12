# Tide web server

[Tide](https://docs.rs/tide/latest/tide/) web server.

## How to run the server

```sh
cargo run
```

## How to test the server

Thera are two endpoints:

- `GET /` - returns a simple message
- `POST /api/users` - accepts a JSON object and validates the property `name`

Calling index endpoint:

```sh
curl http://localhost:8080
```

Expected a string response.

Posting a JSON object:

```sh
curl -X POST -H "Content-Type: application/json" \
-d '{"name": "Alice", "age": 30}' \
http://localhost:8080/api/users
```

Expected a 200 OK response if the JSON object has the property `name`.
