// [2, 4, 0, 1, 3, 7, 6] ==> [[0, 2], [1, 4, 3], [6, 7]]
// Verified by: https://atcoder.jp/contests/joisc2007/submissions/24248388
fn decompose_into_cycles(a: &[usize]) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut vis = vec![false; n];
    let mut ans = vec![];
    for i in 0..n {
        if vis[i] { continue; }
        vis[i] = true;
        let mut cyc = vec![i];
        let mut v = a[i];
        while v != i {
            vis[v] = true;
            cyc.push(v);
            v = a[v];
        }
        ans.push(cyc)
    }
    ans
}
