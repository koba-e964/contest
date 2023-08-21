// Depends on MInt.rs
// Verified by: https://yukicoder.me/submissions/906625
// O(n^3)
fn determinant(a: &[Vec<MInt>]) -> MInt {
    let n = a.len();
    assert_eq!(a[0].len(), n);
    let mut a = a.to_vec();
    let mut ans = MInt::new(1);
    for i in 0..n {
        let mut r = i;
        while r < n && a[r][i] == 0.into() {
            r += 1;
        }
        if r >= n {
            return MInt::new(0);
        }
        if r != i {
            a.swap(r, i);
            ans = -ans;
        }
        let aii = a[i][i];
        let aiiinv = aii.inv();
        a[i][i] = 1.into();
        for j in i + 1..n {
            a[i][j] *= aiiinv;
        }
        ans *= aii;
        for j in i + 1..n {
            let aji = a[j][i];
            a[j][i] = 0.into();
            for k in i + 1..n {
                let val = aji * a[r][k];
                a[j][k] -= val;
            }
        }
    }
    ans
}
