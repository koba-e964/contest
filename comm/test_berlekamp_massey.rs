include!("MInt.rs");
include!("berlekamp_massey.rs");

fn main() {
    // A is a 1x1 matrix.
    let r = MInt::new(15);
    let e = 14412;
    let a = vec![vec![r]];
    let ans = eval_matpow(&a, e, &[MInt::new(1)], &[MInt::new(2)]);
    assert_eq!(ans, r.pow(e) * 2);

    // A is the Fibonacci sequence's transition matrix.
    let a = vec![vec![MInt::new(0), MInt::new(1)], vec![MInt::new(1); 2]];
    let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for i in 0..expected.len() {
        let ans = eval_matpow(
            &a,
            i as i64,
            &[MInt::new(0), MInt::new(1)],
            &[MInt::new(1), MInt::new(0)],
        );
        assert_eq!(ans, expected[i].into());
    }
    // u and v are 0. The resulting sequence is [0, 0, ...].
    let a = vec![vec![MInt::new(2)]];
    let ans = eval_matpow(&a, e, &[MInt::new(0)], &[MInt::new(0)]);
    assert_eq!(ans, 0.into());
}
