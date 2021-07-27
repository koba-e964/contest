fn convolution(a: &[MInt], b: &[MInt]) -> Vec<MInt> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let n = a.len() - 1;
    let m = b.len() - 1;
    let mut ans = vec![MInt::new(0); n + m + 1];
    for i in 0..n + 1 {
        for j in 0..m + 1 {
            ans[i + j] += a[i] * b[j];
        }
    }
    ans
}

// Finds [x^n] p(x)/q(x)
// Ref: https://qiita.com/ryuhe1/items/da5acbcce4ac1911f47a
// Verified by: https://atcoder.jp/contests/tdpc/submissions/24583334
fn bostan_mori(p: &[MInt], q: &[MInt], mut n: i64) -> MInt {
    assert!(p.len() < q.len());
    let mut p = p.to_vec();
    let mut q = q.to_vec();
    while n > 0 {
        let mut qn = q.clone();
        for i in 0..qn.len() {
            if i % 2 == 1 {
                qn[i] = -qn[i];
            }
        }
        let num = convolution(&p, &qn);
        let den = convolution(&q, &qn);
        let mut nxt_p = vec![MInt::new(0); q.len() - 1];
        let mut nxt_q = vec![MInt::new(0); q.len()];
        for i in 0..q.len() - 1 {
            let to = 2 * i + (n % 2) as usize;
            if to < num.len() {
                nxt_p[i] = num[to];
            }
        }
        for i in 0..q.len() {
            nxt_q[i] = den[2 * i];
        }
        p = nxt_p;
        q = nxt_q;
        n /= 2;
    }
    p[0] * q[0].inv()
}
