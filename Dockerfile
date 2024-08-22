FROM rust:1.80 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new whatismyip
WORKDIR /usr/src/whatismyip
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/whatismyip .
RUN ["./whatismyip", "-l"]
USER 1000
ENTRYPOINT ["./whatismyip"]