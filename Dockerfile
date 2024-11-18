FROM rust:1.73
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/raptor"]