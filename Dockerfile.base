FROM rust:latest

USER 0

RUN apt-get update
RUN apt-get install musl-tools cmake g++ llvm clang -y
RUN rustup target add x86_64-unknown-linux-musl

RUN mkdir -p /home/wasi
COPY Cargo.toml Cargo.lock /home/wasi/
COPY src /home/wasi/src
COPY contrib /home/wasi/contrib
COPY s2i /usr/libexec/s2i
#RUN chmod -R 777 /home/wasi

WORKDIR /home/wasi
RUN ln -s "/usr/bin/g++" "/usr/bin/musl-g++"
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
#RUN RUSTFLAGS=-Clinker=musl-gcc cargo install --path . --target=x86_64-unknown-linux-musl
