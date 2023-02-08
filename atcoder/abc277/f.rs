use std::cmp::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Strong connected components.
// This struct uses O(n) stack space.
// Verified by: yukicoder No.470 (http://yukicoder.me/submissions/145785)
//              ABC214-H (https://atcoder.jp/contests/abc214/submissions/25082618)
pub struct SCC {
    n: usize,
    ncc: usize,
    g: Vec<Vec<usize>>, // graph in adjacent list
    rg: Vec<Vec<usize>>, // reverse graph
    cmp: Vec<usize>, // topological order
}

impl SCC {
    pub fn new(n: usize) -> Self {
        SCC {
            n: n,
            ncc: n + 1,
            g: vec![Vec::new(); n],
            rg: vec![Vec::new(); n],
            cmp: vec![0; n],
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.g[from].push(to);
        self.rg[to].push(from);
    }
    fn dfs(&self, v: usize, used: &mut [bool], vs: &mut Vec<usize>) {
        used[v] = true;
        for &w in self.g[v].iter() {
            if !used[w] {
               self.dfs(w, used, vs);
            }
        }
        vs.push(v);
    }
    fn rdfs(&self, v: usize, k: usize,
            used: &mut [bool], cmp: &mut [usize]) {
        used[v] = true;
        cmp[v] = k;
        for &w in self.rg[v].iter() {
            if !used[w] {
                self.rdfs(w, k, used, cmp);
            }
        }
    }
    pub fn scc(&mut self) -> usize {
        let n = self.n;
        let mut used = vec![false; n];
        let mut vs = Vec::new();
        let mut cmp = vec![0; n];
        for v in 0..n {
            if !used[v] { self.dfs(v, &mut used, &mut vs); }
        }
        for u in used.iter_mut() {
            *u = false;
        }
        let mut k = 0;
        for &t in vs.iter().rev() {
            if !used[t] { self.rdfs(t, k, &mut used, &mut cmp); k += 1; }
        }
        self.ncc = k;
        self.cmp = cmp;
        k
    }
    pub fn top_order(&self) -> Vec<usize> {
        assert!(self.ncc <= self.n);
        self.cmp.clone()
    }
    // Returns a dag whose vertices are scc's, and whose edges are those of the original graph.
    pub fn dag(&self) -> Vec<Vec<usize>> {
        assert!(self.ncc <= self.n);
        let ncc = self.ncc;
        let mut ret = vec![vec![]; ncc];
        let n = self.n;
        for i in 0..n {
            for &to in self.g[i].iter() {
                if self.cmp[i] != self.cmp[to] {
                    assert!(self.cmp[i] < self.cmp[to]);
                    ret[self.cmp[i]].push(self.cmp[to]);
                }
            }
        }
        ret.into_iter().map(|mut v| {
            v.sort_unstable(); v.dedup();
            v
        }).collect()
    }
    pub fn rdag(&self) -> Vec<Vec<usize>> {
        assert!(self.ncc <= self.n);
        let ncc = self.ncc;
        let mut ret = vec![vec![]; ncc];
        let n = self.n;
        for i in 0..n {
            for &to in self.g[i].iter() {
                if self.cmp[i] != self.cmp[to] {
                    assert!(self.cmp[i] < self.cmp[to]);
                    ret[self.cmp[to]].push(self.cmp[i]);
                }
            }
        }
        ret.into_iter().map(|mut v| {
            v.sort_unstable(); v.dedup();
            v
        }).collect()
    }
}

// Tags: topological-sort
fn main() {
    input! {
        h: usize, w: usize,
        a: [[i32; w]; h],
    }
    let mut rngs = vec![];
    for i in 0..h {
        let mut mi = 1 << 28;
        let mut ma = 0;
        for j in 0..w {
            if a[i][j] > 0 {
                mi = min(mi, a[i][j]);
                ma = max(ma, a[i][j]);
            }
        }
        if mi <= ma {
            rngs.push((mi, ma, a[i].clone()));
        }
    }
    rngs.sort_by_key(|&(x, y, _)| (x, y));
    let m = rngs.len();
    for i in 1..m {
        if rngs[i - 1].1 > rngs[i].0 {
            println!("No");
            return;
        }
    }
    let mut edges = vec![];
    let mut nv = w;
    for (_, _, row) in rngs {
        let mut coo = vec![];
        for i in 0..w {
            if row[i] > 0 {
                coo.push(row[i]);
            }
        }
        if coo.is_empty() { continue; }
        coo.sort(); coo.dedup();
        let m = coo.len();
        for i in 0..w {
            if row[i] == 0 { continue; }
            let idx = coo.binary_search(&row[i]).unwrap();
            if idx > 0 {
                edges.push((nv + 2 * idx - 1, i));
            }
            if idx + 1 < m {
                edges.push((i, nv + 2 * idx));
            }
        }
        for i in 0..m - 1 {
            edges.push((nv + 2 * i, nv + 2 * i + 1));
        }
        nv += 2 * m - 2;
    }
    let mut scc = SCC::new(nv);
    for (u, v) in edges {
        scc.add_edge(u, v);
    }
    let ncc = scc.scc();
    println!("{}", if ncc == nv { "Yes" } else { "No" });
}
