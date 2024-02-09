FROM ghcr.io/xmtp/rust:latest
ARG CARGO_INCREMENTAL

USER xmtp
ENV USER=xmtp

RUN sudo apt update && sudo apt install -y pkg-config openssl libssl-dev

COPY --from=ghcr.io/xmtp/foundry:latest /usr/local/bin/anvil /usr/local/bin/anvil

ARG PROJECT=xps-gateway
WORKDIR /workspaces/${PROJECT}
COPY --chown=xmtp:xmtp . .

ENV PATH=~${USER}/.cargo/bin:$PATH
ENV USER=xmtp

RUN yamlfmt -lint .github/workflows/*.yml

ENV CARGO_INCREMENTAL=${CARGO_INCREMENTAL:-1}
RUN cargo check
RUN cargo fmt --check
RUN cargo clippy --all-features --no-deps -- -D warnings
RUN cargo test --workspace --all-features
RUN CARGO_TARGET_DIR=/workspaces/${PROJECT}/target cargo install --path xps-gateway --bin=xps_gateway --root=~${USER}/.cargo/
RUN valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes --verbose ~${USER}/.cargo/bin/xps_gateway --help

CMD RUST_LOG=info cargo run -- --host 0.0.0.0 --port 8080

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
