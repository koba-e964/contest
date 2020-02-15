/// Computes block-cut tree (https://en.wikipedia.org/wiki/Biconnected_component#Block-cut_tree).
/// Returns (components, edges of contracted forest, #nodes of forest, is_cutpoint, id_of_component, kind of node in forest)
/// Reference: https://codeforces.com/blog/entry/73910 (A)
/// Depends on: UnionFind.rs, offline_lca.rs
/// Verified by: https://codeforces.com/gym/102512/problem/A
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Cut,
    Block,
}

fn block_cut_tree(
    g: &[Vec<usize>]
) -> (Vec<Vec<usize>>, Vec<(usize, usize)>, usize, Vec<bool>, Vec<usize>, Vec<Kind>) {
    let n = g.len();
    // for 1-vertex-0-edge graphs, handle manually.
    if n == 1 {
        return (vec![vec![0]], vec![], 1, vec![false], vec![0], vec![Kind::Block]);
    }
    let mut dep = vec![0; n];
    let mut low = vec![0; n];
    let mut vis = vec![false; n];
    let mut art = vec![false; n];
    let mut comps = vec![];
    let mut stk = vec![];
    let mut ch = vec![vec![]; n];
    fn dfs1(v: usize, g: &[Vec<usize>], par: usize,
            vis: &mut [bool], dep: &mut [i32], low: &mut [i32], art: &mut [bool],
            ch: &mut [Vec<usize>],
            cnt: &mut i32) {
        assert!(!vis[v]);
        dep[v] = *cnt;
        vis[v] = true;
        *cnt += 1;
        let mut mi = *cnt;
        let mut has_art = false;
        for &w in &g[v] {
            if w == par { continue; }
            if vis[w] {
                mi = std::cmp::min(mi, dep[w]);
            } else {
                ch[v].push(w);
                dfs1(w, g, v, vis, dep, low, art, ch, cnt);
                mi = std::cmp::min(mi, low[w]);
                if low[w] >= dep[v] {
                    has_art = true;
                }
            }
        }
        art[v] = if par >= g.len() { ch[v].len() >= 2 } else { has_art };
        low[v] = mi;
    }
    fn dfs2(v: usize, g: &[Vec<usize>],
            ch: &[Vec<usize>],
            dep: &[i32], low: &[i32], art: &[bool],
            comps: &mut Vec<Vec<usize>>, stk: &mut Vec<usize>,
    ) {
        stk.push(v);
        for &w in &ch[v] {
            dfs2(w, g, ch, dep, low, art, comps, stk);
            if low[w] >= dep[v] {
                let mut last = vec![v];
                while last.last() != Some(&w) {
                    last.push(stk.pop().unwrap());
                }
                comps.push(last);
            }
        }
    }
    let mut cnt = 0;
    for v in 0..n {
        if !vis[v] {
            dfs1(
                v, g, n,
                &mut vis, &mut dep, &mut low, &mut art,
                &mut ch,
                &mut cnt,
            );
            dfs2(
                v, g, &ch,
                &dep, &low, &art,
                &mut comps, &mut stk,
            );
            stk.clear();
        }
    }

    // build the block cut tree
    let mut tree = vec![];
    let mut id = vec![0; n];
    let mut treenode = 0;
    let mut ty = vec![];
    for v in 0..n {
        if art[v] {
            id[v] = treenode;
            treenode += 1;
            ty.push(Kind::Cut);
        }
    }
    for comp in &comps {
        let node = treenode;
        treenode += 1;
        ty.push(Kind::Block);
        for &v in comp {
            if art[v] {
                tree.push((id[v], node));
            } else {
                id[v] = node;
            }
        }
    }
    (comps, tree, treenode, art, id, ty)
}
