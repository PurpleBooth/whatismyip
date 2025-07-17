target "build-environment" {
    dockerfile = "Dockerfile"
    context = "."
}

target "lint" {
    dockerfile-inline = <<EOF
FROM buildenv AS lint
# Build application
COPY . .
RUN cargo fmt --all -- --check
RUN cargo clippy --all-features
RUN cargo check
RUN cargo audit
EOF

    contexts = {
        buildenv = "target:build-environment"
    }
}


target "test" {
    dockerfile-inline = <<EOF
FROM buildenv AS test
COPY . .
RUN cargo test
EOF

    contexts = {
        buildenv = "target:build-environment"
    }
}

target "specdown" {
    dockerfile-inline = <<EOF
FROM buildenv AS specdown
# Build application
COPY . .
RUN cargo build --release
RUN specdown run --temporary-workspace-dir --add-path "/app/target/release" ./README.md
EOF

    contexts = {
        buildenv = "target:build-environment"
    }
}

target "docker" {
    dockerfile-inline = <<EOF
FROM --platform=$BUILDPLATFORM buildenv AS docker
ARG TARGETPLATFORM
# Build application
COPY . .
ENV RUSTFLAGS="-C target-feature=+crt-static"

RUN case "$TARGETPLATFORM" in \
      "linux/amd64") cargo zigbuild --release --target "x86_64-unknown-linux-musl" ;; \
      "linux/arm64") cargo zigbuild --release --target "aarch64-unknown-linux-musl" ;; \
      *) echo "$TARGETPLATFORM not supported" && exit 1 ;; \
    esac
FROM scratch AS final
COPY --from=docker /etc/passwd /etc/passwd
COPY --from=docker "/app/target/release/whatismyip" /whatismyip
ENTRYPOINT ["/whatismyip"]
EOF

    platforms = ["linux/amd64", "linux/arm64"]

    contexts = {
        buildenv = "target:build-environment"
    }

    attest = [
        "type=provenance,mode=max",
        "type=sbom"
    ]
}

target "bins" {
    dockerfile-inline = <<EOF
FROM --platform=$BUILDPLATFORM buildenv AS bins
ARG TARGETPLATFORM
# Build application
COPY . .

RUN case "$TARGETPLATFORM" in \
      "linux/amd64") cargo zigbuild --release --target "aarch64-unknown-linux-gnu" ;; \
      "linux/arm64") cargo zigbuild --release --target "x86_64-unknown-linux-gnu" ;; \
      "windows/amd64") cargo zigbuild --release --target "aarch64-pc-windows-gnu" ;; \
      "windows/arm64") cargo zigbuild --release --target "x86_64-pc-windows-gnu" ;; \
      "darwin") cargo zigbuild --release --target "universal2-apple-darwin" ;; \
      *) echo "$TARGETPLATFORM not supported" && exit 1 ;; \
    esac
FROM scratch AS final
COPY --from=bins "/app/target/release/whatismyip" /whatismyip
EOF

    platforms = [
        "linux/amd64",
        "linux/arm64",
        "windows/amd64",
        "windows/arm64",
        "darwin"
    ]

    contexts = {
        buildenv = "target:build-environment"
    }

    output = [{type="local",dest="target/bins"}]

    attest = [
        "type=provenance,mode=max",
        "type=sbom"
    ]
}

target "packages" {
    dockerfile-inline = <<EOF
FROM --platform=$BUILDPLATFORM buildenv AS packages
ARG TARGETPLATFORM TARGETOS TARGETARCH

# Build application
COPY . .

ENV GOARCH=$TARGETARCH
ENV GOOS=$TARGETOS

RUN case "$TARGETPLATFORM" in \
      "linux/amd64") cargo zigbuild --release --target "aarch64-unknown-linux-gnu" ;; \
      "linux/arm64") cargo zigbuild --release --target "x86_64-unknown-linux-gnu" ;; \
      *) echo "$TARGETPLATFORM not supported" && exit 1 ;; \
    esac && \
    VER="$(yq -o tsv -p toml ".package.version" Cargo.toml)" nfpm pkg --packager archlinux --config="nfpm.yaml" && \
    VER="$(yq -o tsv -p toml ".package.version" Cargo.toml)" nfpm pkg --packager rpm --config="nfpm.yaml" && \
    VER="$(yq -o tsv -p toml ".package.version" Cargo.toml)" nfpm pkg --packager apk --config="nfpm.yaml" && \
    VER="$(yq -o tsv -p toml ".package.version" Cargo.toml)" nfpm pkg --packager deb --config="nfpm.yaml"

FROM scratch AS final
COPY --from=packages /app/*.rpm /
COPY --from=packages /app/*.deb /
COPY --from=packages /app/*.apk /
COPY --from=packages /app/*.zst /
EOF

    platforms = ["linux/amd64", "linux/arm64"]

    contexts = {
        buildenv = "target:build-environment"
    }

    output = [{type="local",dest="target/packages"}]

    attest = [
        "type=provenance,mode=max",
        "type=sbom"
    ]
}
