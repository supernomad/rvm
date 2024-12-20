FROM --platform=${BUILDPLATFORM} rust:1-bookworm AS build

ARG BUILDARCH

RUN rm -f /etc/apt/apt.conf.d/docker-clean

RUN --mount=type=cache,id=${BUILDARCH},target=/var/cache/apt \
    --mount=type=cache,id=${BUILDARCH},target=/var/lib/apt/lists \
    apt-get update -yqq && \
    apt-get install -yqq protobuf-compiler jq

ENV UPX_VERSION "4.2.4"
RUN mkdir upx && cd upx && \
    curl -L https://github.com/upx/upx/releases/download/v${UPX_VERSION}/upx-${UPX_VERSION}-${BUILDARCH}_linux.tar.xz --output upx.tar.xz && \
    tar xvf upx.tar.xz && \
    mv upx-${UPX_VERSION}-${BUILDARCH}_linux/upx /bin/upx && \
    cd .. && rm -rf upx

WORKDIR /opt/rvm

# Now copy in the source code to build and then compress the binary.
COPY . .

ARG TARGETARCH

RUN --mount=type=cache,id=${TARGETARCH},target=/usr/local/cargo/git \
    --mount=type=cache,id=${TARGETARCH},target=/usr/local/cargo/registry \
    --mount=type=cache,id=${TARGETARCH},target=/opt/rvm/target \
    make ${TARGETARCH}-ci

FROM --platform=${TARGETPLATFORM} gcr.io/distroless/static:latest AS runtime

COPY --from=build --chown=nonroot:nonroot --chmod=0700 /opt/rvm/output/rvmd /bin/rvmd

USER nonroot
WORKDIR /home/nonroot
ENTRYPOINT [ "/bin/rvmd" ]
