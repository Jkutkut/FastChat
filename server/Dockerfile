FROM alpine:3.16 as builder

RUN apk add --update	curl \
						gcc \
						musl-dev

# Instalation of rust and cargo
RUN curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y

# Compilation
WORKDIR /app

# Dependencies (cache)
RUN $HOME/.cargo/bin/cargo new fastchat-server
WORKDIR fastchat-server
COPY Cargo.toml Cargo.lock ./
RUN $HOME/.cargo/bin/cargo build --target x86_64-unknown-linux-musl --release

# Build
COPY . .
RUN $HOME/.cargo/bin/cargo build --release

RUN mv ./target/release/fastchat-server /

# TODO Debug
# ENV RUST_BACKTRACE=full

# Final image
FROM alpine:3.16
WORKDIR /app
EXPOSE 4242
COPY --from=builder /fastchat-server .
ENTRYPOINT ["./fastchat-server"]
