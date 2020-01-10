FROM rust:latest as cargo-build

USER 0

RUN apt-get update
RUN apt-get install musl-tools cmake g++ llvm clang -y
RUN rustup target add x86_64-unknown-linux-musl

RUN mkdir -p /home/wasi
COPY Cargo.toml Cargo.lock /home/wasi/
COPY src /home/wasi/src
COPY contrib /home/wasi/contrib
COPY s2i /usr/libexec/s2i
RUN chmod -R 777 /home/wasi

WORKDIR /home/wasi
RUN ln -s "/usr/bin/g++" "/usr/bin/musl-g++"
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/faas-wasm-runtime*

RUN RUSTFLAGS=-Clinker=musl-gcc cargo install --path . --target=x86_64-unknown-linux-musl


FROM alpine:latest

RUN addgroup -g 1000 wasi
RUN adduser -D -s /bin/sh -u 1000 -G wasi wasi

WORKDIR /home/wasi/bin/

COPY --from=cargo-build /home/wasi/target/x86_64-unknown-linux-musl/release/faas-wasm-runtime-image ./wasm-runtime
RUN chown wasi:wasi wasm-runtime
COPY module /home/wasi/module

ENV PORT=8080
EXPOSE $PORT

ENV FUNCTION_NAME=add
ENV MODULE_PATH=/home/wasi/module/add.wasm

USER wasi

CMD ["./wasm-runtime"]
