ARG BUILDKIT_SBOM_SCAN_CONTEXT=true

# Download NFPM
FROM goreleaser/nfpm@sha256:929e1056ba69bf1da57791e851d210e9d6d4f528fede53a55bd43cf85674450c AS nfpm

FROM --platform=$BUILDPLATFORM rust AS base
ARG BUILDKIT_SBOM_SCAN_STAGE=true
ARG TARGETPLATFORM

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
    apt-get clean && \
    rm -vrf /var/lib/apt/lists/*

# Install Zig for cross-compilation
# renovate: datasource=github-tags depName=ziglang/zig versioning=loose
ARG ZIG_VERSION=0.15.0-dev.936+fc2c1883b
RUN wget "https://ziglang.org/builds/zig-x86_64-linux-${ZIG_VERSION}.tar.xz" -O /tmp/zig.tar.xz && \
    mkdir -p /var/opt/zig && \
    tar -xvf /tmp/zig.tar.xz -C /var/opt/zig && \
    rm -vf /tmp/zig.tar.xz && \
    ls /var/opt/zig/
ENV PATH="/var/opt/zig/zig-x86_64-linux-${ZIG_VERSION}:${PATH}"

# Install nfpm for packaging
COPY --from=nfpm /usr/bin/nfpm /usr/bin/nfpm

# Install yq for YAML processing
# renovate: datasource=github-releases depName=mikefarah/yq
ARG YQ_VERSION=4.40.5
ARG YQ_BINARY=yq_linux_amd64
RUN wget https://github.com/mikefarah/yq/releases/download/v${YQ_VERSION}/${YQ_BINARY}.tar.gz -O - | \
    tar -xvz && mv ${YQ_BINARY} /usr/local/bin/yq

# Drop to an unprivilaged user
ENV HOME=/home/nonroot
ENV PATH=/home/nonroot/.cargo/bin:$PATH
RUN addgroup nonroot && \
    adduser --disabled-password --ingroup nonroot nonroot && \
    mkdir -p /app /home/nonroot/.cargo/bin/ && \
    chown -vR nonroot:nonroot /app /home/nonroot
USER nonroot
WORKDIR /app

# Install cargo binstall
# renovate: datasource=crate depName=cargo-binstall
ARG CARGO_BINSTALL_VERSION=1.14.1
RUN cargo install cargo-binstall --version ${CARGO_BINSTALL_VERSION} --locked

# Install specdown
# renovate: datasource=github-releases depName=specdown/specdown
ARG SPECDOWN_VERSION=1.2.112
RUN TEMP_SRC="$(mktemp -d)" && \
    git clone https://github.com/specdown/specdown.git "$TEMP_SRC" && \
    cd "$TEMP_SRC" && \
    git switch --detach "v${SPECDOWN_VERSION}" && \
    cargo build --release && \
    mkdir -p '/home/nonroot/.cargo/bin/' && \
    cp -v target/release/specdown /home/nonroot/.cargo/bin/specdown && \
    cd / && \
    rm -vrf "$TEMP_SRC" && \
    specdown --version

# Install cargo-chef for dependency caching
# renovate: datasource=crate depName=cargo-chef
ARG CARGO_CHEF_VERSION=0.1.72
RUN cargo binstall cargo-chef --version ${CARGO_CHEF_VERSION} --locked

# Install cargo-audit for security auditing
# renovate: datasource=crate depName=cargo-audit
ARG CARGO_AUDIT_VERSION=0.21.2
RUN cargo binstall cargo-audit --version ${CARGO_AUDIT_VERSION} --locked

# Install cargo-zigbuild for security auditing
# renovate: datasource=crate depName=cargo-zigbuild
ARG CARGO_ZIGBUILD_VERSION=0.20.0
RUN cargo binstall cargo-zigbuild --version ${CARGO_ZIGBUILD_VERSION} --locked

# Generate build dep list
FROM --platform=$BUILDPLATFORM base AS planner
ARG TARGETPLATFORM
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM --platform=$BUILDPLATFORM base AS builder
ARG TARGETPLATFORM

# Build dependencies with cross-compilation
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Add targets
RUN rustup target add \
      aarch64-apple-darwin \
      aarch64-pc-windows-gnullvm \
      aarch64-unknown-linux-gnu \
      aarch64-unknown-linux-musl \
      x86_64-apple-darwin \
      x86_64-pc-windows-gnu \
      x86_64-unknown-linux-gnu \
      x86_64-unknown-linux-musl

# Build application
COPY . .
