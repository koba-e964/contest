use std::io::{Write, BufWriter};
use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

/// Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
/// Many-rooted version.
/// Verified by: https://yukicoder.me/submissions/714482
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
    pub fn ancestor(&self, v: usize, dist: usize) -> usize {
        let mut ans = v;
        for i in 0..self.bn + 1 {
            if (dist & 1 << i) != 0 {
                ans = self.lca_tbl[ans][i];
            }
        }
        ans
    }
    #[allow(unused)]
    pub fn child(&self, anc: usize, des: usize) -> usize {
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

fn dfs(v: usize, par: usize, g: &[Vec<usize>], ban: &[bool]) -> bool {
    if ban[v] {
        return false;
    }
    let mut ch = 0;
    for &w in &g[v] {
        if w == par { continue; }
        if dfs(w, v, g, ban) {
            return true;
        }
        ch += 1;
    }
    ch == 0
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let k: usize = get();
        let x: Vec<usize> = (0..k).map(|_| get::<usize>() - 1).collect();
        let mut g = vec![vec![]; n];
        for _ in 0..n - 1 {
            let a = get::<usize>() - 1;
            let b = get::<usize>() - 1;
            g[a].push(b);
            g[b].push(a);
        }
        let lca = LCA::new(&g, &[0]);
        let mut ban = vec![false; n];
        for &x in &x {
            let d = lca.depth(x);
            let d = d / 2;
            let y = lca.ancestor(x, d);
            ban[y] = true;
        }
        puts!("{}\n", if dfs(0, n, &g, &ban) { "Yes" } else { "No" });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
