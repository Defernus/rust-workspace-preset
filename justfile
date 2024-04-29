set export
set dotenv-load

default:
    @just --list

run:
    cargo run --bin generate-random-value

fmt:
    cargo fmt --all

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

audit:
    cargo audit

# TODO: add tests
# test: