set -eux

cd `dirname $0`

RUST_TESTERS="test_pollard_rho test_berlekamp_massey math/test_determinant"
for file in ${RUST_TESTERS}; do
    rustc ${file}.rs -O -o ${file}
    ./${file}
done

