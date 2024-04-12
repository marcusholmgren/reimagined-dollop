# Simple web server

This is a simple web server that only uses the standard library. It is not a framework.

## How to run

```sh
cargo run
```

Expecte that the server is running on `http://localhost:8080`.

## How to test

Get the index page

```sh
curl http://localhost:8080
```

Expecte that the index page is returned.

Get the second page

```sh
curl http://localhost:8080/second
```

Expecte that the second page is returned.
