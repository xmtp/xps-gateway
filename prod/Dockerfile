FROM ghcr.io/xmtp/rust:latest as builder

USER xmtp

RUN sudo apt update && sudo apt install -y pkg-config openssl libssl-dev

RUN sudo mkdir -p /xmtp
RUN sudo chown -R xmtp:xmtp /xmtp

WORKDIR /build
COPY --chown=xmtp:xmtp . .

ENV PATH=~xmtp/.cargo/bin:$PATH
ENV USER=xmtp

RUN cargo check
RUN cargo fmt --check
RUN cargo clippy --all-features --no-deps -- -D warnings
RUN cargo test
RUN cargo build --release
RUN CARGO_TARGET_DIR=/build/target cargo install --root /xmtp --path xps-gateway --bin=xps_gateway
RUN strip /xmtp/bin/xps_gateway
RUN cp CHANGELOG.md /xmtp/
RUN cp LICENSE /xmtp/

FROM debian:stable-slim
ARG TARGETARCH

RUN export DEBIAN_FRONTEND=noninteractive && \
  apt update && \
  apt install -y -q --no-install-recommends \
    ca-certificates apt-transport-https \
    && \
  apt clean && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /xmtp/ /xmtp/

RUN useradd -s /dev/null none

EXPOSE 8080

ENV USER=none
USER none

CMD ["/xmtp/bin/xps_gateway", "--host", "0.0.0.0", "--port", "8080"]

LABEL org.label-schema.build-date=$BUILD_DATE \
    org.label-schema.name="xps-gateway" \
    org.label-schema.description="xps-gateway" \
    org.label-schema.url="https://github.com/xmtp/xps-gateway" \
    org.label-schema.vcs-ref=$VCS_REF \
    org.label-schema.vcs-url="git@github.com:xmtp/xps-gateway.git" \
    org.label-schema.vendor="xmtp" \
    org.label-schema.version=$VERSION \
    org.label-schema.schema-version="1.0" \
    org.opencontainers.image.description="XPS-Gateway Container"
