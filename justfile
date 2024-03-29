set export
set dotenv-load

default:
    @just --list

run:
    cargo run --bin bot

fmt:
    cargo fmt --all

fix-fmt:
    cargo fmt --all -- --check

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

fix-clippy:
    cargo clippy --fix --all-targets --all-features -- -D warnings

audit:
    cargo audit

# TODO: add tests
# test: