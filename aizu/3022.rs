// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Ported and modified from https://kokiymgch.hatenablog.com/entry/2018/03/21/174958
// This library uses O(n) stack space.

//tree index : [0, 1, ..., bc.size() - 1] -> component
//tree index : [bc.size(), bc.size() + 1, ..., bc.size() + art.size() - 1] -> articulation point
//cmp[index of edge] -> index of the node of the constructed tree
//cmp_node[index of node] -> None if it's not an articulation point, otherwise index of the node of the constructed tree
// if Some(y) = cmp_node(x), y >= bc
// Example: 0-1-2 (e = [(0, 1), (1, 2)])
// cmp = [0, 1]
// cmp_node = [None, Some(2), None]
// tree = [[2], [2], [0, 1]]
// bc == 2, art.size() == 1
pub struct Bicomp {
    pub cmp: Vec<usize>,
    pub cmp_node: Vec<Option<usize>>,
    pub g: Vec<Vec<usize>>,
    pub e2i: std::collections::HashMap<(usize, usize), usize>,
    pub tree: Vec<Vec<usize>>,
    pub bc: usize,
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
                    tmp.push((u.min(v), u.max(v)))
                }
                if !used[v] {
                    cnt += 1;
                    Self::dfs(v, u, used, ord, low, k, tmp, bc, art, g);
                    low[u] = low[u].min(low[v]);
                    if prev < n && low[v] >= ord[u] {
                        is_art = true;
                    }
                    if low[v] >= ord[u] {
                        bc.push(vec![]);
                        loop {
                            let e = tmp.pop().unwrap();
                            let idx = bc.len() - 1;
                            bc[idx].push(e);
                            if (u.min(v), u.max(v)) == e {
                                break;
                            }
                        }
                    }
                } else {
                    low[u] = low[u].min(ord[v]);
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
            e2i.insert((u.min(v), u.max(v)), i);
        }
        let mut cmp_node = vec![None; n];
        let mut cmp = vec![!0; m];
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
                let idx = e2i[&(e.0.min(e.1), e.0.max(e.1))];
                cmp[idx] = i;
            }
        }
        let mut tree = vec![vec![]; bc.len() + art.len()];
        let mut tmp = vec![];
        for i in 0..art.len() {
            let j = i + bc.len();
            cmp_node[art[i]] = Some(j);
            let u = art[i];
            for &v in &g[u] {
                let t = cmp[e2i[&(u.min(v), u.max(v))]];
                tmp.push(t);
            }
            tmp.sort_unstable(); tmp.dedup();
            for v in tmp.drain(..) {
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
            bc: bc.len(),
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, par: usize, g: &[Vec<usize>], a: &[i64], dp: &mut [i64]) -> i64 {
    let mut s = a[v];
    for &w in &g[v] {
        if w == par { continue; }
        s += dfs(w, v, g, a, dp);
    }
    dp[v] = s;
    s
}

fn dfs2(v: usize, par: usize, g: &[Vec<usize>], bc: usize, a: &[i64], dp: &[i64], res: &mut [i64]) {
    let mut rem = dp[0] - a[v];
    let mut ma = 0;
    for &w in &g[v] {
        if w == par { continue };
        dfs2(w, v, g, bc, a, dp, res);
        if v >= bc {
            rem -= dp[w];
            ma = ma.max(dp[w]);
        }
    }
    res[v] = rem.max(ma);
}

// Tags: articulation-points, biconnected-components, block-cut-trees
// Similar problems: https://yukicoder.me/problems/no/1326
fn solve() {
    input! {
        n: usize, m: usize,
        w: [i64; n],
        uv: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        let (u, v) = uv[i];
        g[u].push(i);
        g[v].push(i);
    }
    let bct = Bicomp::new(n, &uv);
    let mut dp = vec![0; bct.tree.len()];
    let mut w_tree = vec![0; bct.tree.len()];
    for i in 0..n {
        if let Some(idx) = bct.cmp_node[i] {
            w_tree[idx] += w[i];
        } else {
            let idx = bct.cmp[g[i][0]];
            w_tree[idx] += w[i];
        }
    }
    dfs(0, bct.tree.len(), &bct.tree, &w_tree, &mut dp);
    let mut res = vec![0; bct.tree.len()];
    dfs2(0, bct.tree.len(), &bct.tree, bct.bc, &w_tree, &dp, &mut res);
    for i in 0..n {
        if let Some(x) = bct.cmp_node[i] {
            println!("{}", res[x]);
        } else {
            println!("{}", dp[0] - w[i]);
        }
    }
}
