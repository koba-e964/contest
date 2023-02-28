// Returns (root, children)
// This functions uses O(n) stack space.
// Complexity: O(n log n)-time, O(n)-space
// Verified by: ABC291-Ex (https://atcoder.jp/contests/abc291/submissions/39303290)
fn centroid_decompose(g: &[Vec<usize>]) -> (usize, Vec<Vec<usize>>) {
    fn find_subtree_sizes(g: &[Vec<usize>], v: usize, par: usize,
                          dp: &mut [usize], vis: &[bool]) {
        let mut sum = 1;
        for &w in &g[v] {
            if par == w || vis[w] { continue; }
            find_subtree_sizes(g, w, v, dp, vis);
            sum += dp[w];
        }
        dp[v] = sum;
    }
    fn centroid_decompose_inner(g: &[Vec<usize>], v: usize, par: usize,
                                ch: &mut [Vec<usize>],
                                dp: &mut [usize], vis: &mut [bool]) -> usize {
        let n = g.len();
        find_subtree_sizes(g, v, n, dp, vis);
        let cent = {
            let sz = dp[v];
            let find_centroid = |mut v: usize, mut par: usize| {
                loop {
                    let mut has_majority = false;
                    for &w in &g[v] {
                        if par == w || vis[w] { continue; }
                        if dp[w] > sz / 2 {
                            par = v;
                            v = w;
                            has_majority = true;
                            break;
                        }
                    }
                    if !has_majority {
                        return v;
                    }
                }
            };
            find_centroid(v, n)
        };
        if par < n {
            ch[par].push(cent);
        }
        // v was selected as a centroid
        // and will be ignored in the following decomposition procedure
        vis[cent] = true;
        for &w in &g[cent] {
            if !vis[w] {
                centroid_decompose_inner(g, w, cent, ch, dp, vis);
            }
        }
        cent
    }
    let n = g.len();
    let mut ch = vec![vec![]; n];
    // This Vec is used across multiple calls to `centroid_decompose_inner`
    let mut dp = vec![0; n];
    let mut vis = vec![false; n];
    let root = centroid_decompose_inner(&g, 0, n, &mut ch, &mut dp, &mut vis);
    (root, ch)
}
