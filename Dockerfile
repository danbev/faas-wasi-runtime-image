FROM dbevenius/wasm-base-image:latest as cargo-build

COPY src /home/wasi/src

WORKDIR /home/wasi
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN RUSTFLAGS=-Clinker=musl-gcc cargo install --force --path . --target=x86_64-unknown-linux-musl

FROM alpine:latest

RUN addgroup -g 1000 wasi && adduser -D -s /bin/sh -u 1000 -G wasi wasi && chown -R wasi:wasi /home/wasi && mkdir -p /home/wasi/bin
RUN chmod -R 777 /home/wasi/bin
USER wasi

WORKDIR /home/wasi/bin/

COPY --from=cargo-build /home/wasi/target/x86_64-unknown-linux-musl/release/faas-wasm-runtime-image ./wasm-runtime
RUN chown wasi:wasi wasm-runtime
COPY module /home/wasi/module

ENV PORT=8080
EXPOSE $PORT

ENV FUNCTION_NAME=add
ENV MODULE_PATH=/home/wasi/module/add.wasm

CMD ["./wasm-runtime"]
