// Verified by https://atcoder.jp/contests/arc080/submissions/3957276
fn bipartite_matching(g: &[Vec<bool>]) -> usize {
    let n = g.len();
    if n == 0 { return 0; }
    let m = g[0].len();
    let mut to = vec![None; m];
    let mut visited = vec![false; n];
    let mut ans = 0;
    fn augment(v: usize, g: &[Vec<bool>],
               visited: &mut [bool], to: &mut [Option<usize>])
               -> bool {
        if visited[v] { return false; }
        visited[v] = true;
        for i in 0 .. g[v].len() {
            if !g[v][i] { continue; }
            if let Some(w) = to[i] {
                if augment(w, g, visited, to) {
                    to[i] = Some(v); return true;
                }
            } else {
                to[i] = Some(v); return true;
            }
        }
        false
    }
    for i in 0 .. n {
        for v in visited.iter_mut() { *v = false; }
        if augment(i, &g, &mut visited, &mut to) { ans += 1; }
    }
    ans
}
