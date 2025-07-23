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
ARG TARGETOS
ARG TARGETARCH
ENV TARGETPLATFORM=$TARGETPLATFORM
ENV TARGETOS=$TARGETOS
ENV TARGETARCH=$TARGETARCH
# Build application
COPY . .
RUN cross-platform-build
FROM scratch AS final
COPY --from=docker /etc/passwd /etc/passwd
COPY --from=docker "/app/target/release/whatismyip" /whatismyip
USER nonroot
ENTRYPOINT ["/whatismyip"]
EOF

    platforms = ["alpine/amd64", "alpine/arm64"]

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
ARG TARGETOS
ARG TARGETARCH
ENV TARGETPLATFORM=$TARGETPLATFORM
ENV TARGETOS=$TARGETOS
ENV TARGETARCH=$TARGETARCH

# Build application
COPY . .

RUN cross-platform-build
FROM scratch AS final
COPY --from=bins "/app/target/release/whatismyip" /whatismyip
EOF

    platforms = [
        "linux/amd64",
        "linux/arm64",
        "windows/amd64",
        "windows/arm64",
        "darwin/amd64",
        "darwin/arm64"
    ]

    contexts = {
        buildenv = "target:build-environment",
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

RUN cross-platform-build && \
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
