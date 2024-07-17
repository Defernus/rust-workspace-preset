set export
set dotenv-load

default:
    @just --list

run:
    cargo run --bin generate-random-value

lint:
    cargo deny check --hide-inclusion-graph  --allow duplicate --allow advisory-not-detected
    cargo fmt --all --check
    cargo check
    cargo clippy

test:
    cargo nextest run --run-ignored default

test-integration:
    cargo nextest run --run-ignored ignored-only

test-all:
    cargo nextest run --run-ignored all