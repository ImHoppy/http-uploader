# Build

FROM alpine:3.16.0 AS build

RUN set -ex; \
    apk add --no-cache \
        clang=13.0.1-r1 \
        make=4.3-r0

WORKDIR /app

COPY srcs/ srcs/
COPY Makefile Makefile

RUN make


# Run

FROM alpine:3.16.0

RUN set -ex; \
    apk add --no-cache \
    libstdc++=11.2.1_git20220219-r2

RUN addgroup -S runner && adduser -S runner -G runner
USER runner

COPY --chown=runner:runner --from=build \
    ./app/fb_cdn \
    ./app/

ENTRYPOINT [ "./app/fb_cdn" ]

EXPOSE 8080
