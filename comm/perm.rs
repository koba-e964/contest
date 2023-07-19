fn perm_comp(a: &[usize], b: &[usize]) -> Vec<usize> {
    let n = a.len();
    assert_eq!(b.len(), n);
    let mut c = vec![0; n];
    for i in 0..n {
        c[i] = a[b[i]];
    }
    c
}

fn perm_inv(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut c = vec![0; n];
    for i in 0..n {
        c[a[i]] = i;
    }
    c
}

fn perm_exp(a: &[usize], mut e: i64) -> Vec<usize> {
    let n = a.len();
    let mut cur: Vec<_> = (0..n).collect();
    let mut prod = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            prod = perm_comp(&cur, &prod)
        }
        cur = perm_comp(&cur, &cur);
        e /= 2;
    }
    prod
}

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
