FROM rust:1.85
WORKDIR /app
COPY Cargo.toml ./
COPY lib.rs ./lib.rs
COPY bin ./bin
RUN cargo build --release
ENTRYPOINT ["cargo", "run", "--release", "--bin"]
