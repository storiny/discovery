dev:
    npm run build && cargo watch -w src -x run | bunyan

dev_raw:
    npm run build && cargo watch -w src -x run

build:
    npm run build && cargo build --release

build_img:
    npm run build && docker build --platform linux/arm64 -t storiny_discovery .

fmt:
    cargo fmt

test:
    npm run build && cargo nextest run --workspace

test_ci:
    npm run build && cargo nextest run --no-fail-fast --workspace

test_verbose:
    npm run build && cargo nextest run --no-capture --no-fail-fast --workspace

udeps:
    cargo +nightly udeps --all-targets
