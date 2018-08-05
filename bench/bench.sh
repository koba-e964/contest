set -eu
rustup run nightly rustc --test -O $1.rs
RUST_TEST_THREADS=1 ./$1
