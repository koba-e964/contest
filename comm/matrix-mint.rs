// Depends on MInt.rs
// Verified by: https://atcoder.jp/contests/abc199/submissions/22259436
fn squmul(a: &[Vec<MInt>], b: &[Vec<MInt>]) -> Vec<Vec<MInt>> {
    let n = a.len();
    let mut ret = vec![vec![MInt::new(0); n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ret[i][k] += a[i][j] * b[j][k];
            }
        }
    }
    ret
}

fn squpow(a: &[Vec<MInt>], mut e: i64) -> Vec<Vec<MInt>> {
    let n = a.len();
    let mut sum = vec![vec![MInt::new(0); n]; n];
    for i in 0..n { sum[i][i] = 1.into(); }
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            sum = squmul(&sum, &cur);
        }
        cur = squmul(&cur, &cur);
        e /= 2;
    }
    sum
}
