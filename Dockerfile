ARG BUILDKIT_SBOM_SCAN_CONTEXT=true

# Base stages
FROM goreleaser/nfpm@sha256:929e1056ba69bf1da57791e851d210e9d6d4f528fede53a55bd43cf85674450c AS nfpm

FROM --platform=$BUILDPLATFORM tonistiigi/xx@sha256:923441d7c25f1e2eb5789f82d987693c47b8ed987c4ab3b075d6ed2b5d6779a3 AS xx
ARG TARGETPLATFORM

FROM --platform=$BUILDPLATFORM rust:alpine@sha256:ec0413a092f4cc01b32e08f991485abe4467ef95c7416a6643a063a141c2e0ec AS chef
ARG BUILDKIT_SBOM_SCAN_STAGE=true
ARG TARGETPLATFORM
RUN apk add clang lld openssl-dev curl bash
# copy xx scripts to your build stage
COPY --from=xx / /
RUN xx-apk add --no-cache musl-dev zlib-dev zlib-static openssl-dev openssl-libs-static pkgconfig alpine-sdk

WORKDIR /app

# Install cargo-chef for dependency caching
RUN cargo install cargo-chef --locked

# Planner stage
FROM chef AS planner
ARG TARGETPLATFORM
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM chef AS builder
ARG TARGETPLATFORM
ARG VER
ENV VER=$VER

# Copy nfpm for packaging
COPY --from=nfpm /usr/bin/nfpm /usr/bin/nfpm

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies with cross-compilation
RUN xx-cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN xx-cargo build --release --target-dir ./build && \
    xx-verify --static "./build/$(xx-cargo --print-target-triple)/release/whatismyip" && \
    cp -v "./build/$(xx-cargo --print-target-triple)/release/whatismyip" ./build/whatismyip

# Always build packages (but only copy them in bins stage)
RUN mkdir -p /PACKS && \
    GOARCH="$(xx-info arch)" GOOS="$(xx-info os)" nfpm pkg --packager archlinux --config="nfpm.yaml" --target="/PACKS" && \
    GOARCH="$(xx-info arch)" GOOS="$(xx-info os)" nfpm pkg --packager rpm --config="nfpm.yaml" --target="/PACKS" && \
    GOARCH="$(xx-info arch)" GOOS="$(xx-info os)" nfpm pkg --packager apk --config="nfpm.yaml" --target="/PACKS" && \
    GOARCH="$(xx-info arch)" GOOS="$(xx-info os)" nfpm pkg --packager deb --config="nfpm.yaml" --target="/PACKS"

# Users stage for container build
FROM --platform=$BUILDPLATFORM alpine@sha256:8a1f59ffb675680d47db6337b49d22281a139e9d709335b492be023728e11715 AS users
RUN addgroup -S nonroot && adduser -S nonroot -G nonroot

# Bins output stage (includes packages)
FROM scratch AS bins
USER nonroot
COPY --from=builder /PACKS .
COPY --from=builder /app/build/whatismyip .

# Container runtime stage (only binary, no packages)
FROM scratch AS container
COPY --from=users /etc/passwd /etc/passwd
COPY --from=builder /app/build/whatismyip /usr/local/bin/
USER nonroot
ENTRYPOINT ["/usr/local/bin/whatismyip"]
