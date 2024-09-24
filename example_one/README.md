# Example One

This example demonstrates the use of Actix Web to create a simple API that returns a "pong" message when the `/api/ping` endpoint is accessed. It also shows how to use the `Responder` trait to return a JSON response.

## Running the Example

To run this example, you need to have Rust and Cargo installed on your machine. Then, navigate to the example's directory and run:

```bash
cargo run
```

This will start the server on `http://127.0.0.1:8080`. You can access the "pong" message by navigating to `http://127.0.0.1:8080/api/ping` in your web browser.
