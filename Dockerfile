FROM rust:1.80 AS builder
ARG TARGETPLATFORM
USER 1000
WORKDIR /usr/src/

RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then  \
    rustup target add x86_64-unknown-linux-musl;  \
    elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then  \
    rustup target add armv7-unknown-linux-musleabihf;  \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then  \
    rustup target add aarch64-unknown-linux-musl;  \
    else exit 1;  \
    fi

WORKDIR /usr/src/whatismyip
COPY . ./

RUN --mount=type=cache,target=/usr/src/whatismyip/target \
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then  \
    cargo build --target=x86_64-unknown-linux-musl --release ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then  \
    cargo build --target=armv7-unknown-linux-musleabihf --release ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then  \
    cargo build --target=aarch64-unknown-linux-musl --release ;  \
    else exit 1;  \
    fi

RUN --mount=type=cache,target=/usr/src/whatismyip/target \
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then  \
    cargo install --target=x86_64-unknown-linux-musl --path . ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then  \
    cargo install --target=armv7-unknown-linux-musleabihf --path . ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then  \
    cargo install --target=aarch64-unknown-linux-musl --path . ;  \
    else exit 1;  \
    fi

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/whatismyip .
RUN ["./whatismyip", "-l"]
USER 1000
ENTRYPOINT ["./whatismyip"]
