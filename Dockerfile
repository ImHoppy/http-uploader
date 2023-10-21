# Build

FROM alpine:3.17.0 AS build

RUN set -ex; \
    apk add --no-cache \
        clang15=15.0.7-r0 \
        make=4.3-r1 \
        g++=12.2.1_git20220924-r4

WORKDIR /app

COPY srcs/ srcs/
COPY Makefile Makefile

RUN make


# Run

FROM alpine:3.17.0

RUN set -ex; \
    apk add --no-cache \
    libstdc++-dev=12.2.1_git20220924-r4

RUN addgroup -S runner && adduser -S runner -G runner
USER runner

# COPY --chown=runner:runner --from=build \
COPY --from=build \
    ./app/fb_cdn \
    ./app/

EXPOSE 8080

ENTRYPOINT [ "./app/fb_cdn" ]
