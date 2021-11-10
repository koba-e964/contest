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

/// Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
/// Many-rooted version.
/// Verified by: https://yukicoder.me/submissions/413634
pub struct LCA {
    n: usize,
    bn: usize,
    parent: Vec<usize>, // r is root <=> parent[r] = r
    dep: Vec<usize>,
    lca_tbl: Vec<Vec<usize>>
}

impl LCA {
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
        for i in 1..self.bn + 1 {
            for v in 0..n {
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
        for l in (0..self.bn + 1).rev() {
            if dy - dx >= 1 << l {
                y = self.lca_tbl[y][l];
                dy -= 1 << l;
            }
        }
        assert_eq!(dx, dy);
        
        if x == y {
            return x;
        }
        for l in (0..self.bn + 1).rev() {
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
    #[allow(unused)]
    pub fn parent(&self, a: usize) -> usize {
        self.parent[a]
    }
    #[allow(unused)]
    fn child(&self, anc: usize, des: usize) -> usize {
        let x = self.dep[des] - self.dep[anc] - 1;
        let mut ans = des;
        for i in 0..self.bn + 1 {
            if (x & 1 << i) != 0 {
                ans = self.lca_tbl[ans][i];
            }
        }
        ans
    }
    pub fn new(edges: &[Vec<usize>], roots: &[usize]) -> Self {
        let n = edges.len();
        let bn = (n.next_power_of_two() - 1).count_ones() as usize;
        let mut ret = LCA {
            n: n, bn: bn, parent: vec![0; n], dep: vec![0; n],
            lca_tbl: vec![Vec::new(); n] };
        for &r in roots {
            ret.dfs(edges, r, r, 0);
        }
        ret.lca_init();
        ret
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, par: usize, gpar: usize, a: &[i64], g: &[Vec<usize>],
       dp1: &mut [i32], dp2: &mut [i32], dpeq: &mut [i32],
       x1: i32, x2: i32, xeq: i32) {
    let n = g.len();
    let a1 = if par == n || a[par] > a[v] { 0 } else { 1 };
    let a2 = if par == n || a[par] < a[v] { 0 } else { 1 };
    let aeq = if gpar == n || a[gpar] != a[v] { 0 } else { 1 };
    dp1[v] = x1 + a1;
    dp2[v] = x2 + a2;
    dpeq[v] = xeq + aeq;
    for &w in &g[v] {
        if par == w { continue; }
        dfs(w, v, par, a, g, dp1, dp2, dpeq, x2 + a2, x1 + a1, xeq + aeq);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [i64; n],
        xy: [(usize1, usize1); n - 1],
        q: usize,
        uv: [(usize1, usize1); q],
    }
    let mut g = vec![vec![]; n];
    for &(x, y) in &xy {
        g[x].push(y);
        g[y].push(x);
    }
    let lca = LCA::new(&g, &[0]);
    let mut dp1 = vec![0; n];
    let mut dp2 = vec![0; n];
    let mut dpeq = vec![0; n];
    dfs(0, n, n, &a, &g, &mut dp1, &mut dp2, &mut dpeq, 0, 0, 0);
    let ac = |anc: usize, ch: usize, des: usize| -> bool {
        let x = lca.dep[des] - lca.dep[anc];
        dpeq[des] == dpeq[ch] && if x % 2 == 0 {
            dp1[des] == dp1[anc] || dp2[des] == dp2[anc]
        } else {
            dp1[des] == dp2[anc] || dp2[des] == dp1[anc]
        }
    };
    let is_kado = |v: usize, x: usize, y: usize| {
        a[x] != a[y] && (a[x] - a[v]) * (a[y] - a[v]) > 0
    };
    for &(u, v) in &uv {
        let l = lca.lca(u, v);
        let (mut ok, x, y) = if u == l || v == l {
            let other = u + v - l;
            let ch = lca.child(l, other);
            let par = lca.parent[other];
            if par == l {
                (false, 0, 0)
            } else if u == l {
                (ac(l, ch, other), ch, par)
            } else {
                (ac(l, ch, other), par, ch)
            }
        } else {
            let chu = lca.child(l, u);
            let chv = lca.child(l, v);
            let is_kado = is_kado(l, chu, chv);
            (ac(l, chu, u) && ac(l, chv, v) && is_kado, lca.parent[u], lca.parent[v])
        };
        if ok {
            if !is_kado(u, x, v) || !is_kado(v, y, u) {
                ok = false;
            }
        }
        puts!("{}\n", if ok { "YES" } else { "NO" });
    }
}
