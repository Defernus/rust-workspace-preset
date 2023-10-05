FROM rust:1.72.0

WORKDIR /generate-random-value

COPY . .
RUN cargo install --path ./generate_random_value

CMD ["generate-random-value"]
