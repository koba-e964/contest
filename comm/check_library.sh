set -eux

cd `dirname $0`

RUST_TESTERS="test_pollard_rho"
for file in ${RUST_TESTERS}; do
    rustc ${file}.rs -O
    ./${file}
done

