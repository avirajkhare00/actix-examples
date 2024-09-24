# Example Two

This example demonstrates how to read a JSON file and parse it into a Rust struct using Actix Web. It also shows how to use the `web::Data` extractor to pass data between handlers and the `Mutex` to ensure thread-safe access to the data.

## Running the Example

To run this example, you need to have Rust and Cargo installed on your machine. Then, navigate to the example's directory and run:

```bash
cargo run
```

This will start the server on `http://127.0.0.1:8080`. You can access the user profile by navigating to `http://127.0.0.1:8080/api/profile/john_doe` in your web browser.
