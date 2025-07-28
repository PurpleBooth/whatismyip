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
COPY --from=bins "/app/target/release/whatismyip*" /
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

target "build-homebrew-formula" {
    dockerfile-inline = <<EOF
FROM homebrew/brew:latest AS build-homebrew-formula
USER root

# renovate: datasource=github-releases depName=mikefarah/yq
ARG YQ_VERSION=4.43.1
RUN curl -L https://github.com/mikefarah/yq/releases/download/v$${YQ_VERSION}/yq_linux_amd64.tar.gz -o - | \
  tar xz && mv yq_linux_amd64 /usr/local/bin/yq

RUN apt-get update && \
    apt-get install -y gettext && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

USER linuxbrew
# Accept GITHUB_REPOSITORY from environment or use default
ARG GITHUB_REPOSITORY="PurpleBooth/whatismyip"

# Copy the repository
COPY homebrew/formula.rb.envsubstr homebrew/formula.rb.envsubstr
COPY Cargo.toml Cargo.toml

# Generate the formula from template
RUN VERSION=$(yq -o tsv -p toml ".package.version" Cargo.toml) && \
    TEMP_DIR="$(mktemp -d)" && \
    curl --silent --fail --output "$TEMP_DIR/v$VERSION.tar.gz" \
    "https://codeberg.org/$GITHUB_REPOSITORY/archive/v$VERSION.tar.gz" || \
    touch "$TEMP_DIR/v$VERSION.tar.gz" && \
    FILE_SHA="$(sha256sum --binary "$TEMP_DIR/v$VERSION.tar.gz" | cut -d' ' -f1)" && \
    export VERSION FILE_SHA GITHUB_REPOSITORY GITHUB_REF_NAME="v$VERSION" && \
    envsubst < homebrew/formula.rb.envsubstr > "whatismyip.rb"

FROM scratch AS final
COPY --from=build-homebrew-formula /home/linuxbrew/whatismyip.rb /whatismyip.rb
EOF

    output = [{type="local",dest="target/homebrew"}]
}

target "lint-homebrew-formula" {
    dockerfile-inline = <<EOF
FROM build-homebrew-formula AS formula-source

FROM homebrew/brew:latest AS lint-homebrew-formula
USER linuxbrew

# Copy the formula file from the build-homebrew-formula target
COPY --from=formula-source /whatismyip.rb /home/linuxbrew/whatismyip.rb

# Create a new Homebrew tap for testing
RUN brew tap-new homebrew-releaser/test --no-git && \
    # Copy the formula file into the test tap's Formula directory
    cp -vr /home/linuxbrew/*.rb "$(brew --repository)/Library/Taps/homebrew-releaser/homebrew-test/Formula/" && \
    # Lint each formula file in the test tap
    for file in "$(brew --repository)/Library/Taps/homebrew-releaser/homebrew-test/Formula/"*.rb; do \
      filename=$(basename "$file") && \
      formula_name=$(echo "$filename" | sed 's/\.rb$//') && \
      brew audit --formula "homebrew-releaser/test/$formula_name" || exit 1; \
    done && \
    # Remove the test tap after completion
    brew untap homebrew-releaser/test
EOF

    contexts = {
        "build-homebrew-formula" = "target:build-homebrew-formula"
    }
}

