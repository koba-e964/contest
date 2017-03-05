#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
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

/**
 * Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
 * Verified by: 
 */
pub struct LowestCommonAncestor {
    n: usize,
    bn: usize,
    parent: Vec<usize>, // 0 is root, parent[0] = 0
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
    pub fn parent(&self, a: usize) -> usize {
        self.parent[a]
    }
    pub fn new(edges: &[Vec<usize>]) -> Self {
        let n = edges.len();
        let bn = (n.next_power_of_two() - 1).count_ones() as usize;
        let mut ret = LowestCommonAncestor {
            n: n, bn: bn, parent: vec![0; n], dep: vec![0; n],
            lca_tbl: vec![Vec::new(); n] };
        ret.dfs(edges, 0, 0, 0);
        ret.lca_init();
        ret
    }
}

fn dfs(v: usize, par: Option<usize>, edges: &[Vec<usize>], dp: &mut [i64]) {
    let mut tot = 0;
    for &w in edges[v].iter() {
        if Some(w) == par { continue; }
        dfs(w, Some(v), edges, dp);
        let child = dp[w];
        tot += child;
    }
    dp[v] += tot;
}

fn solve() {
    let n = get();
    let mut edges = vec![Vec::new(); n];
    for _ in 0 .. n - 1 {
        let u: usize = get::<usize>() - 1;
        let v: usize = get::<usize>() - 1;
        edges[u].push(v);
        edges[v].push(u);
    }
    let lca = LowestCommonAncestor::new(&edges);
    let q = get();
    let mut dp = vec![0; n];
    for _ in 0 .. q {
        let a: usize = get::<usize>() - 1;
        let b: usize = get::<usize>() - 1;
        let anc = lca.lca(a, b);
        dp[a] += 1;
        dp[b] += 1;
        dp[anc] -= 1;
        if anc != lca.parent(anc) {
            dp[lca.parent(anc)] -= 1;
        }
    }
    // Recover full trace (imos)
    dfs(0, None, &edges, &mut dp);
    let mut tot = 0;
    for ent in dp {
        tot += ent * (ent + 1) / 2;
    }
    println!("{}", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
