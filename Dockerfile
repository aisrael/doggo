FROM clux/muslrust AS builder
RUN mkdir -p /usr/src/ &&\
    cd /usr/src &&\
    rustup target add x86_64-unknown-linux-musl &&\
    USER=root cargo new doggo
WORKDIR /usr/src/doggo
COPY Cargo.* ./
RUN cargo build --target x86_64-unknown-linux-musl && cargo build --target x86_64-unknown-linux-musl --release

COPY . .
RUN find src -type f -exec touch {} \+ &&\
    cargo build --target x86_64-unknown-linux-musl --release

FROM busybox:1.32.0-musl
COPY --from=builder /usr/src/doggo/target/x86_64-unknown-linux-musl/release/doggo .
ENTRYPOINT [ "/doggo" ]
