export RUST_BACKTRACE=1;

cargo fmt;
cargo test -- --show-output --no-capture;