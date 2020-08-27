FROM busybox:1.32.0-musl
COPY target/x86_64-unknown-linux-musl/release/doggo /usr/bin/doggo
ENTRYPOINT [ "/usr/bin/doggo" ]
