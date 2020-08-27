.PHONY: docker musl-binary

head = $(shell git rev-parse HEAD)

docker: musl-binary
	export HASH=$(head); \
	docker build \
	-t registry.gitlab.com/aisrael/doggo:$${HASH:0:8} \
	-t registry.gitlab.com/aisrael/doggo:debug-$${HASH:0:8} \
	-t registry.gitlab.com/aisrael/doggo:latest .

musl-binary: target/x86_64-unknown-linux-musl/release/doggo

target/x86_64-unknown-linux-musl/release/doggo:
	docker run -v --rm $$(pwd):/src clux/muslrust:1.45.2 \
	sh -c 'cd /src && cargo build --target x86_64-unknown-linux-musl --release'
