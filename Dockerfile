FROM rust:1.80@sha256:29fe4376919e25b7587a1063d7b521d9db735fc137d3cf30ae41eb326d209471 AS builder
ARG TARGETPLATFORM
USER 1000
RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then  \
    rustup target add x86_64-unknown-linux-musl;  \
    elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then  \
    rustup target add armv7-unknown-linux-musleabihf;  \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then  \
    rustup target add aarch64-unknown-linux-musl;  \
    else exit 1;  \
    fi

WORKDIR /app/whatismyip
COPY . ./

RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then  \
    cargo build --target=x86_64-unknown-linux-musl --release ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then  \
    cargo build --target=armv7-unknown-linux-musleabihf --release ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then  \
    cargo build --target=aarch64-unknown-linux-musl --release ;  \
    else exit 1;  \
    fi

# Bundle Stage
FROM scratch
COPY --from=builder /app/whatismyip/target/*/release/whatismyip .
RUN ["./whatismyip", "-l"]
USER 1000
ENTRYPOINT ["./whatismyip"]
