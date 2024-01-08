FROM ghcr.io/xmtp/rust:latest
USER xmtp

RUN sudo apt update && sudo apt install -y pkg-config openssl libssl-dev

ARG PROJECT=xps-gateway
WORKDIR /workspaces/${PROJECT}
COPY --chown=xmtp:xmtp . .

ENV PATH=~xmtp/.cargo/bin:$PATH
ENV USER=xmtp

RUN ~xmtp/.cargo/bin/cargo check
RUN ~xmtp/.cargo/bin/cargo fmt --check
RUN ~xmtp/.cargo/bin/cargo clippy --all-features --no-deps
RUN ~xmtp/.cargo/bin/cargo test

LABEL org.label-schema.build-date=$BUILD_DATE \
    org.label-schema.name="rustdev" \
    org.label-schema.description="Rust Development Container" \
    org.label-schema.url="https://github.com/xmtp/gateway" \
    org.label-schema.vcs-ref=$VCS_REF \
    org.label-schema.vcs-url="git@github.com:xmtp/gateway.git" \
    org.label-schema.vendor="xmtp" \
    org.label-schema.version=$VERSION \
    org.label-schema.schema-version="1.0" \
    org.opencontainers.image.description="Rust Development Container"
