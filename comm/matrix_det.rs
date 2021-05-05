// Verified by: Japanese Student Championship 2021 (G)
// https://atcoder.jp/contests/jsc2021/submissions/22334292
// Depends on: MInt.rs
fn matrix_det(a: &[Vec<MInt>]) -> MInt {
    let n = a.len();
    let mut a = a.to_vec();
    // TODO no pivoting
    let mut det = MInt::new(1);
    for i in 0..n {
        let mut r = i;
        while r < n && a[r][i] == 0.into() {
            r += 1;
        }
        if r >= n {
            return 0.into();
        }
        if r != i {
            a.swap(r, i);
            det = -det;
        }
        let aii = a[i][i];
        a[i][i] = 1.into();
        det *= aii;
        for j in i + 1..n {
            a[i][j] *= aii.inv();
        }
        for j in 0..n {
            if i == j { continue; }
            let ajc = a[j][i];
            a[j][i] = 0.into();
            for k in i + 1..n {
                let val = ajc * a[i][k];
                a[j][k] -= val;
            }
        }
    }
    det
}
