This is a simple server with two endpoints:
 - `/upload` - accepts a file upload via FormData and stores it in the `uploads` directory, returning the file hash
 - `/file/<hash>` - returns the file with the given hash

## Build
You will need rust and cargo.
To run the server, run `cargo run` in the root directory. You can also change the default port of 8080 via the env variable `PORT`.
