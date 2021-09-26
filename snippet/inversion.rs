// Inversion count between possibly not pairwise distinct sequences.
// If a and b are not equal as multisets, -1 is returned.
// Otherwise, the inversion is returned.
// Verified by: https://onlinejudge.u-aizu.ac.jp/services/review.html#ACPC2021Day3/5913773
// Depends on: datastr/SegTree.rs
fn inversion(a: &[i64], b: &[i64]) -> i64 {
    let n = a.len();
    let mut c = a.to_vec();
    c.sort();
    let mut d = b.to_vec();
    d.sort();
    if c != d {
        return -1;
    }
    c.dedup();
    let m = c.len();
    let mut occa = vec![vec![]; m];
    let mut occb = vec![vec![]; m];
    for i in 0..a.len() {
        occa[c.binary_search(&a[i]).unwrap()].push(i);
        occb[c.binary_search(&b[i]).unwrap()].push(i);
    }
    let mut trans = vec![0; n];
    for i in 0..m {
        for j in 0..occa[i].len() {
            trans[occb[i][j]] = occa[i][j];
        }
    }
    let mut st = SegTree::new(n, |x, y| x + y, 0);
    let mut ans = 0i64;
    for i in 0..n {
        ans += st.query(trans[i] + 1, n);
        st.update(trans[i], 1);
    }
    ans
}
