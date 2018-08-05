set -eu
rustup run nightly rustc --test -C opt-level=3 $1.rs
RUST_TEST_THREADS=1 ./$1
