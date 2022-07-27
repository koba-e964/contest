// Verified by: https://codeforces.com/contest/700/submission/165850980
// This function uses O(n) stack space.
mod tecomp {
    use std::cmp::min;
    const INF: usize = 1 << 28;
    fn dfs<EIdx: Copy + PartialEq>(v: usize, paridx: EIdx, g: &[Vec<(usize, EIdx)>],
           ord: &mut [usize], low: &mut [usize], k: &mut usize,
           bridges: &mut Vec<EIdx>) {
        ord[v] = *k;
        low[v] = *k;
        *k += 1;
        for &(w, eidx) in g[v].iter() {
            if paridx == eidx { continue; }
            if ord[w] < ord[v] {
                low[v] = min(low[v], ord[w]);
            } else if ord[w] == INF {
                dfs(w, eidx, g, ord, low, k, bridges);
                low[v] = min(low[v], low[w]);
                if ord[v] < low[w] {
                    bridges.push(eidx);
                }
            }
        }
    }
    fn dfs_comp<EIdx: Copy>(
        v: usize, g: &[Vec<(usize, EIdx)>],
        ord: &[usize], low: &[usize],
        cur_becomp: usize, becomp_count: &mut usize, becomp: &mut [usize], tree: &mut [Vec<(usize, EIdx)>],
        vis: &mut [bool],
    ) {
        becomp[v] = cur_becomp;
        vis[v] = true;
        for &(w, eidx) in g[v].iter() {
            if ord[w] > ord[v] && !vis[w] {
                if ord[v] < low[w] {
                    *becomp_count += 1;
                    tree[cur_becomp].push((*becomp_count, eidx));
                    dfs_comp(w, g, ord, low, *becomp_count, becomp_count, becomp, tree, vis);
                } else {
                    dfs_comp(w, g, ord, low, cur_becomp, becomp_count, becomp, tree, vis);
                }
            }
        }
    }

    type EIdx = usize;

    // Returns (the number of 2-edge connected components, [the id of the component v belongs to | v <- [0 .. g.len()]], the resulting tree, the ids of bridges).
    // Graphs are given and provided in the adjacent list format. (to, edge_id).
    // The provided tree has its own vertex ids, but edge ids are reused.
    // This function uses O(n) stack space.
    pub fn decomp(g: &[Vec<(usize, EIdx)>])
                 -> (usize, Vec<usize>, Vec<Vec<(usize, EIdx)>>, Vec<EIdx>) {
        let n_vert = g.len();
        let mut ord = vec![INF; n_vert];
        let mut low = vec![INF; n_vert];
        let mut k = 0;
        let mut becomp_count = 0;
        let mut becomp = vec![INF; n_vert];
        let mut bridges = Vec::new();
        // rooted forest
        let mut tree = vec![Vec::new(); n_vert];
        let mut vis = vec![false; n_vert];
        for i in 0 .. n_vert {
            if !vis[i] {
                dfs(i, !0, &g, &mut ord, &mut low, &mut k, &mut bridges);
                dfs_comp(i, &g, &ord, &low,
                         becomp_count, &mut becomp_count, &mut becomp,
                         &mut tree, &mut vis);
                becomp_count += 1;
            }
        }
        tree = tree[..becomp_count].to_vec();
        (becomp_count, becomp, tree, bridges)
    }
} // mod tecomp
