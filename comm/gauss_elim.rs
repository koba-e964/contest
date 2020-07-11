// Verified by: AIsing Programming Contest 2020
// https://atcoder.jp/contests/aising2020/submissions/15174348
// Depends on: ModInt.rs
fn gauss_elim(a: &[Vec<ModInt>], b: &[ModInt]) -> Vec<ModInt> {
    let n = a.len();
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    let mut c = 0;
    // TODO no pivoting
    for i in 0..n {
        while c < n && a[i][c] == 0.into() {
            c += 1;
        }
        if c >= n {
            panic!();
        }
        let aic = a[i][c];
        a[i][c] = 1.into();
        for j in c + 1..n {
            a[i][j] *= aic.inv();
        }
        b[i] *= aic.inv();
        for j in 0..n {
            if i == j { continue; }
            let ajc = a[j][c];
            a[j][c] = 0.into();
            for k in c + 1..n {
                let val = ajc * a[i][k];
                a[j][k] -= val;
            }
            let val = ajc * b[i];
            b[j] -= val;
        }
    }
    b
}
