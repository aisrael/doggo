ARG CI_PROJECT_DIR=.
FROM busybox:1.32.0-musl
RUN echo CI_PROJECT_DIR => ${CI_PROJECT_DIR}
COPY $CI_PROJECT_DIR/target/x86_64-unknown-linux-musl/release/doggo /
ENTRYPOINT [ "/doggo" ]
