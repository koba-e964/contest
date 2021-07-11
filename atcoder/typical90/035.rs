use std::io::{Write, BufWriter};
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
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

/// Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
/// Many-rooted version.
/// Verified by: https://yukicoder.me/submissions/413634
pub struct LowestCommonAncestor {
    n: usize,
    bn: usize,
    parent: Vec<usize>, // r is root <=> parent[r] = r
    dep: Vec<usize>,
    lca_tbl: Vec<Vec<usize>>
}

impl LowestCommonAncestor {
    fn dfs(&mut self, edges: &[Vec<usize>], v: usize, par: usize, d: usize) {
        self.parent[v] = par;
        self.dep[v] = d;
        for &u in edges[v].iter() {
            if u != par {
                self.dfs(edges, u, v, d + 1);
            }
        }
    }
    fn lca_init(&mut self) {
        let n = self.n;
        for v in 0 .. n {
            self.lca_tbl[v] = vec![0; self.bn + 1];
            self.lca_tbl[v][0] = self.parent[v];
        }
        for i in 1 .. self.bn + 1 {
            for v in 0 .. n {
                self.lca_tbl[v][i] =
                    self.lca_tbl[self.lca_tbl[v][i - 1]][i - 1];
            }
        }
    }
    pub fn lca(&self, mut x: usize, mut y: usize) -> usize {
        let dx = self.dep[x];
        let mut dy = self.dep[y];
        if dx > dy {
            return self.lca(y, x);
        }
        for l in (0 .. self.bn + 1).rev() {
            if dy - dx >= 1 << l {
                y = self.lca_tbl[y][l];
                dy -= 1 << l;
            }
        }
        assert_eq!(dx, dy);
        
        if x == y {
            return x;
        }
        for l in (0 .. self.bn + 1).rev() {
            if self.lca_tbl[x][l] != self.lca_tbl[y][l] {
	        x = self.lca_tbl[x][l];
	        y = self.lca_tbl[y][l];
            }
        }
        self.lca_tbl[x][0]
    }
    #[allow(unused)]
    pub fn depth(&self, a: usize) -> usize {
        self.dep[a]
    }
    pub fn new(edges: &[Vec<usize>], roots: &[usize]) -> Self {
        let n = edges.len();
        let bn = (n.next_power_of_two() - 1).count_ones() as usize;
        let mut ret = LowestCommonAncestor {
            n: n, bn: bn, parent: vec![0; n], dep: vec![0; n],
            lca_tbl: vec![Vec::new(); n] };
        for &r in roots {
            ret.dfs(edges, r, r, 0);
        }
        ret.lca_init();
        ret
    }
}

fn dfs(v: usize, par: usize, g: &[Vec<usize>],
       rng: &mut [(usize, usize)], cnt: &mut usize) {
    let now = *cnt;
    *cnt += 1;
    for &w in &g[v] {
        if w == par { continue; }
        dfs(w, v, g, rng, cnt);
    }
    rng[v] = (now, *cnt);
}

// The author read the editorial before implementing.
// Tags: euler-tour, lowest-common-ancestor
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
        v: [[usize1]],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut cnt = 0;
    let mut rng = vec![(0, 0); n];
    dfs(0, n, &g, &mut rng, &mut cnt);
    let mut inv = vec![0; n];
    for i in 0..n {
        inv[rng[i].0] = i;
    }
    let lca = LowestCommonAncestor::new(&g, &[0]);
    for v in &v {
        let k = v.len();
        let mut tr = vec![0; k];
        for i in 0..k {
            tr[i] = rng[v[i]].0;
        }
        tr.sort();
        for i in 0..k {
            tr[i] = inv[tr[i]];
        }
        let mut cur = tr[0];
        let mut anc = tr[0];
        let mut tot = 0;
        for i in 1..k {
            let l = lca.lca(cur, tr[i]);
            let l2 = lca.lca(l, anc);
            // cur -> l
            let danc = lca.depth(anc);
            let dl = lca.depth(l);
            if danc > dl {
                tot += danc - dl;
            }
            // l -> tr[i]
            tot += lca.depth(tr[i]) - dl;
            // update
            cur = tr[i];
            anc = l2;
        }
        puts!("{}\n", tot);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
