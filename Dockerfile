FROM --platform=$BUILDPLATFORM tonistiigi/xx@sha256:0c6a569797744e45955f39d4f7538ac344bfb7ebf0a54006a0a4297b153ccf0f AS xx
ARG TARGETPLATFORM

FROM --platform=$BUILDPLATFORM rust:alpine@sha256:466dc9924d265455aa73e72fd9cdac9db69ce6a988e6f0e6baf852db3485d97d AS builder
RUN apk add clang lld openssl-dev
# copy xx scripts to your build stage
COPY --from=xx / /
ARG TARGETPLATFORM

RUN xx-apk add --no-cache musl-dev zlib-dev zlib-static openssl-dev openssl-libs-static pkgconfig alpine-sdk

WORKDIR /app
RUN cargo new --lib whatismyip
WORKDIR /app/whatismyip
COPY Cargo.* ./

RUN xx-cargo build --release --target-dir ./build
COPY . ./
RUN xx-cargo build --release --target-dir ./build && \
    xx-verify --static "./build/$(xx-cargo --print-target-triple)/release/whatismyip" && \
    cp -v  "./build/$(xx-cargo --print-target-triple)/release/whatismyip" "./build/whatismyip"
RUN addgroup -g 568 nonroot
RUN adduser -u 568 -G nonroot -D nonroot
USER nonroot

FROM scratch
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

USER nonroot
COPY --from=builder /app/whatismyip/build/whatismyip .
RUN ["/whatismyip", "--version"]
ENTRYPOINT ["/whatismyip"]
