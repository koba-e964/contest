// 2-SAT solver.
// n: the number of variables (v_1, ..., v_n)
// banned: prohibited combinations
// (if ((i1, v1), (i2, v2)) is in banned, x[i1] == v1 && x[i2] == v2 does not hold.)
// Returns: None if there's no assignment that satisfies banned.
// Otherwise, it returns an assignment that safisfies banned. (true: true, false: false)
// Depends on: graph/SCC.rs
// Verified by: ABC277-Ex (https://atcoder.jp/contests/abc277/submissions/38516994)
fn two_sat(n: usize, banned: &[((usize, bool), (usize, bool))]) -> Option<Vec<bool>> {
    let mut scc = SCC::new(2 * n);
    for &((c1, v1), (c2, v2)) in banned {
        let x = if !v1 {
            c1 + n
        } else {
            c1
        };
        let y = if !v2 {
            c2
        } else {
            c2 + n
        };
        scc.add_edge(x, y);
        scc.add_edge((y + n) % (2 * n), (x + n) % (2 * n));
    }
    scc.scc();
    let mut result = vec![false; n];
    let top_ord = scc.top_order();
    for i in 0 .. n {
        if top_ord[i] == top_ord[i + n] {
            return None;
        }
        result[i] = top_ord[i] > top_ord[i + n];
    }
    Some(result)
}
