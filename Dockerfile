FROM rust:1.72.0

WORKDIR /generate-random-value

COPY . .
RUN cargo install --path ./crates/generate-random-value

CMD ["generate-random-value"]
