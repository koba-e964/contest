/// Tarjan's offline LCA algorithm.
/// https://github.com/spaghetti-source/algorithm/blob/master/graph/least_common_ancestor_tarjan.cc
/// g should represent a forest and roots should be a list of vertices,
/// each of which is the designated root of exactly one connected component. 
/// qs is a Vec
/// where qs[i] should contain the i-th query (a, b),
/// meaning the LCA of a and b is asked for.
/// This function returns out: Vec<usize>,
/// i-th of which contains the output for qs[i].
/// Depends on: UnionFind.rs
/// Verified by: https://yukicoder.me/submissions/413752
fn offline_lca(
    g: &[Vec<usize>],
    roots: &[usize],
    qs: &[(usize, usize)]
) -> Vec<usize> {
    fn visit(g: &[Vec<usize>], u: usize, w: usize,
             q_map: &[Vec<(usize, usize)>],
             col: &mut [bool], out: &mut [usize],
             anc: &mut [usize], uf: &mut UnionFind) {
        for &v in &g[u] {
            if v == w { continue; }
            visit(g, v, u, q_map, col, out, anc, uf);
            uf.unite(u, v);
            anc[uf.root(u)] = u;
        }
        col[u] = true;
        for &(target, idx) in &q_map[u] {
            if col[target] {
                out[idx] = anc[uf.root(target)]
            }
        }
        anc[uf.root(u)] = u;
    }
    let n = g.len();
    let mut uf = UnionFind::new(n);
    let mut col = vec![false; n];
    let mut anc = vec![0; n];
    let mut q_map = vec![vec![]; n];
    for i in 0..qs.len() {
        let (a, b) = qs[i];
        q_map[a].push((b, i));
        q_map[b].push((a, i));
    }
    let mut out = vec![usize::max_value(); qs.len()];
    for &r in roots {
        visit(g, r, n, &q_map, &mut col, &mut out, &mut anc, &mut uf);
    }
    out
}
