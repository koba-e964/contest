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
    #[allow(unused)]
    pub fn parent(&self, a: usize) -> usize {
        self.parent[a]
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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 3 * 104_857_600; // 300 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(g: &[Vec<usize>], dp: &mut [i64], v: usize, par: usize) -> i64 {
    let mut s = dp[v];
    for &w in &g[v] {
        if w == par { continue; }
        s += dfs(g, dp, w, v);
    }
    dp[v] = s;
    s
}

fn solve() {
    {
        let n = 800_000;
        let mut g = vec![vec![]; n];
        for i in 0..n - 1 {
            g[i].push(i + 1);
            g[i + 1].push(i);
        }
        let mut dp = vec![1; n];
        dfs(&g, &mut dp, 0, n);
        eprintln!("done {}", dp[0]);
    }
    let t: usize = get();
    for case_nr in 0..t {
        let n: usize = get();
        let mut g = vec![vec![]; n];
        for _ in 0..n - 1 {
            let a = get::<usize>() - 1;
            let b = get::<usize>() - 1;
            g[a].push(b);
            g[b].push(a);
        }
        let f: Vec<usize> = (0..n).map(|_| get::<usize>() - 1).collect();
        let mut occ = vec![vec![]; n];
        for i in 0..n {
            occ[f[i]].push(i);
        }
        let lca = LowestCommonAncestor::new(&g, &[0]);
        let mut dp = vec![0; n];
        for i in 0..n {
            if occ[i].is_empty() { continue; }
            for j in 0..occ[i].len() - 1 {
                let x = occ[i][j];
                let y = occ[i][j + 1];
                let l = lca.lca(x, y);
                dp[x] += 1;
                dp[y] += 1;
                dp[l] -= 2;
            }
        }
        dfs(&g, &mut dp, 0, n);
        let mut ans = 0;
        for i in 1..n {
            if dp[i] == 0 { ans += 1; }
        }
        println!("Case #{}: {}", case_nr + 1, ans);
    }
}
