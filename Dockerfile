FROM rust:1.80 AS builder
ARG TARGETPLATFORM
WORKDIR /usr/src/

RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then  \
    rustup target add x86_64-unknown-linux-musl;  \
    elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then  \
    rustup target add armv7-unknown-linux-musleabihf;  \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then  \
    rustup target add aarch64-unknown-linux-musl;  \
    else exit 1;  \
    fi

RUN USER=root cargo new whatismyip
WORKDIR /usr/src/whatismyip
COPY Cargo.toml Cargo.lock ./

RUN if [ "$TARGETPLATFORM" = "linux/amd64" ] ; then  \
      cargo build --release --target=x86_64-unknown-linux-musl ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm/v7" ] ; then  \
      cargo build --release --target=armv7-unknown-linux-musleabihf ;  \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ] ; then  \
      cargo build --release --target=aarch64-unknown-linux-musl ;  \
    else exit 1 ;  \
    fi


COPY src ./src

RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then  \
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
