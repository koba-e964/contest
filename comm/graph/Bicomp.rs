// Ported from https://kokiymgch.hatenablog.com/entry/2018/03/21/174958
// This library uses O(n) stack space. 
// Verified by: yukicoder No. 1326 (https://yukicoder.me/submissions/908252)

//tree index : [0, 1, ..., bc.len() - 1] -> component
//tree index : [bc.len(), bc.len() + 1, ..., bc.len() + art.len() - 1] -> articulation point
//cmp[index of edge] -> index of the node of the constructed tree
//cmp_node[index of node] -> None if it's not an articulation point, otherwise index of the node of the constructed tree
pub struct Bicomp {
    pub cmp: Vec<Option<usize>>,
    pub cmp_node: Vec<Option<usize>>,
    pub g: Vec<Vec<usize>>,
    pub e2i: std::collections::HashMap<(usize, usize), usize>,
    pub tree: Vec<Vec<usize>>,
}

impl Bicomp {
    fn dfs(
        u: usize, prev: usize,
        used: &mut [bool],
        ord: &mut [usize],
        low: &mut [usize],
        k: &mut usize,
        tmp: &mut Vec<(usize, usize)>,
        bc: &mut Vec<Vec<(usize, usize)>>,
        art: &mut Vec<usize>,
        g: &[Vec<usize>],
    ) {
        let n = g.len();
        used[u] = true;
        ord[u] = *k;
        *k += 1;
        low[u] = ord[u];
        let mut is_art = false;
        let mut cnt = 0;
        for &v in &g[u] {
            if v != prev {
                if ord[v] >= n || ord[v] < ord[u] {
                    tmp.push((std::cmp::min(u, v), std::cmp::max(u, v)))
                }
                if !used[v] {
                    cnt += 1;
                    Self::dfs(v, u, used, ord, low, k, tmp, bc, art, g);
                    low[u] = std::cmp::min(low[u], low[v]);
                    if prev < n && low[v] >= ord[u] {
                        is_art = true;
                    }
                    if low[v] >= ord[u] {
                        bc.push(vec![]);
                        loop {
                            let e = tmp.pop().unwrap();
                            let idx = bc.len() - 1;
                            bc[idx].push(e);
                            if (std::cmp::min(u, v), std::cmp::max(u, v)) == e {
                                break;
                            }
                        }
                    }
                } else {
                    low[u] = std::cmp::min(low[u], ord[v]);
                }
            }
        }
        if prev >= n && cnt > 1 {
            is_art = true;
        }
        if is_art {
            art.push(u);
        }
    }
    pub fn new(n: usize, edges: &[(usize, usize)]) -> Self {
        let m = edges.len();
        let mut g = vec![vec![]; n];
        let mut e2i = std::collections::HashMap::new();
        for i in 0..m {
            let (u, v) = edges[i];
            g[u].push(v);
            g[v].push(u);
            e2i.insert((std::cmp::min(u, v), std::cmp::max(u, v)), i);
        }
        let mut cmp_node = vec![None; n];
        let mut cmp = vec![None; m];
        let mut ord = vec![n; n];
        let mut low = vec![n; n];
        let mut used = vec![false; n];
        let mut bc = vec![];
        let mut art = vec![];
        let mut tmp = vec![];
        let mut k = 0;

        // build() in original
        Self::dfs(0, n, &mut used, &mut ord, &mut low, &mut k, &mut tmp, &mut bc, &mut art, &g);
        for i in 0..bc.len() {
            for &e in &bc[i] {
                let idx = e2i[&(std::cmp::min(e.0, e.1), std::cmp::max(e.1, e.0))];
                cmp[idx] = Some(i);
            }
        }
        let mut tree = vec![vec![]; bc.len() + art.len()];
        for i in 0..art.len() {
            let j = i + bc.len();
            cmp_node[art[i]] = Some(j);
            let u = art[i];
            let mut tmp = std::collections::HashSet::new();
            for &v in &g[u] {
                let t = cmp[e2i[&(std::cmp::min(u, v), std::cmp::max(v, u))]].unwrap();
                tmp.insert(t);
            }
            for v in tmp {
                tree[j].push(v);
                tree[v].push(j);
            }
        }
        Bicomp {
            cmp: cmp,
            cmp_node: cmp_node,
            g: g,
            e2i: e2i,
            tree: tree,
        }
    }
}
