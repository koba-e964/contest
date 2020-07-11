include!("ModInt.rs");
include!("berlekamp_massey.rs");

fn main() {
    // A is a 1x1 matrix.
    let r = ModInt::new(15);
    let e = 14412;
    let a = vec![vec![r]];
    let ans = eval_matpow(&a, e, &[ModInt::new(1)], &[ModInt::new(2)]);
    assert_eq!(ans, r.pow(e) * 2);

    // A is the Fibonacci sequence's transition matrix.
    let a = vec![vec![ModInt::new(0), ModInt::new(1)], vec![ModInt::new(1); 2]];
    let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for i in 0..expected.len() {
        let ans = eval_matpow(
            &a,
            i as i64,
            &[ModInt::new(0), ModInt::new(1)],
            &[ModInt::new(1), ModInt::new(0)],
        );
        assert_eq!(ans, expected[i].into());
    }
}
