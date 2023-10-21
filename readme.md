This is a simple server with two endpoints:
 - `/upload` - accepts a file upload via FormData and stores it in the `uploads` directory, returning the file hash
 - `/file/<hash>` - returns the file with the given hash

## Build
You will ned clang++ and make to build the server.
To build the server, run `make` in the root directory. This will create a binary called `fb_cdn`.

## Docker
You can also build the server using docker. To do so, run `docker build -t fb_cdn .` in the root directory.
This will create a docker image called `fb_cdn`.
```
docker run \
	--mount type=bind,source="$(pwd)/uploads",target=/app/uploads \
	-p 8080:8080 \
	-d cdn
```
