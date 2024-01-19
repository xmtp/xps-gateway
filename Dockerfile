FROM ghcr.io/xmtp/rust:latest
USER xmtp

RUN sudo apt update && sudo apt install -y pkg-config openssl libssl-dev

ARG PROJECT=xps-gateway
WORKDIR /workspaces/${PROJECT}
COPY --chown=xmtp:xmtp . .

ENV PATH=~xmtp/.cargo/bin:$PATH
ENV USER=xmtp

RUN cargo check
RUN cargo fmt --check
RUN cargo clippy --all-features --no-deps -- -D warnings
RUN cargo test
RUN cargo build --release
RUN CARGO_TARGET_DIR=/workspaces/${PROJECT}/target cargo install --path xps-gateway --bin=xps_gateway
RUN valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes --verbose ${HOME}/.cargo/bin/xps_gateway --help

LABEL org.label-schema.build-date=$BUILD_DATE \
    org.label-schema.name="rustdev" \
    org.label-schema.description="Rust Development Container" \
    org.label-schema.url="https://github.com/xmtp/xps-gateway" \
    org.label-schema.vcs-ref=$VCS_REF \
    org.label-schema.vcs-url="git@github.com:xmtp/xps-gateway.git" \
    org.label-schema.vendor="xmtp" \
    org.label-schema.version=$VERSION \
    org.label-schema.schema-version="1.0" \
    org.opencontainers.image.description="Rust Development Container"
