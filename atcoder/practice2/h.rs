#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
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
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

/**
 * Strong connected components.
 * Verified by: yukicoder No.470 (http://yukicoder.me/submissions/145785)
 */
struct SCC {
    n: usize,
    ncc: usize,
    g: Vec<Vec<usize>>, // graph in adjacent list
    rg: Vec<Vec<usize>>, // reverse graph
    cmp: Vec<usize>, // topological order
}

impl SCC {
    fn new(n: usize) -> Self {
        SCC {
            n: n,
            ncc: n + 1,
            g: vec![Vec::new(); n],
            rg: vec![Vec::new(); n],
            cmp: vec![0; n],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize) {
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
    fn scc(&mut self) -> usize {
        let n = self.n;
        let mut used = vec![false; n];
        let mut vs = Vec::new();
        let mut cmp = vec![0; n];
        for v in 0 .. n {
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
    #[allow(dead_code)]
    fn top_order(&self) -> Vec<usize> {
        assert!(self.ncc <= self.n);
        self.cmp.clone()
    }
    /*
     * Returns a dag whose vertices are scc's, and whose edges are those of the original graph.
     */
    #[allow(dead_code)]
    fn dag(&self) -> Vec<Vec<usize>> {
        assert!(self.ncc <= self.n);
        let ncc = self.ncc;
        let mut ret = vec![HashSet::new(); ncc];
        let n = self.n;
        for i in 0 .. n {
            for &to in self.g[i].iter() {
                if self.cmp[i] != self.cmp[to] {
                    assert!(self.cmp[i] < self.cmp[to]);
                    ret[self.cmp[i]].insert(self.cmp[to]);
                }
            }
        }
        ret.into_iter().map(|set| set.into_iter().collect()).collect()
    }
    #[allow(dead_code)]
    fn rdag(&self) -> Vec<Vec<usize>> {
        assert!(self.ncc <= self.n);
        let ncc = self.ncc;
        let mut ret = vec![HashSet::new(); ncc];
        let n = self.n;
        for i in 0 .. n {
            for &to in self.g[i].iter() {
                if self.cmp[i] != self.cmp[to] {
                    assert!(self.cmp[i] < self.cmp[to]);
                    ret[self.cmp[to]].insert(self.cmp[i]);
                }
            }
        }
        ret.into_iter().map(|set| set.into_iter().collect()).collect()
    }
}

/**
 * 2-SAT solver.
 * n: the number of variables (v_1, ..., v_n)
 * cons: constraints, given in 2-cnf
 * i (1 <= i <= n) means v_i, -i (1 <= i <= n) means not v_i.
 * Returns: None if there's no assignment that satisfies cons.
 * Otherwise, it returns an assignment that safisfies cons. (true: true, false: false)
 * Dependencies: SCC.rs
 * Verified by: Codeforces #400 D
 *              (http://codeforces.com/contest/776/submission/24957215)
 */
fn two_sat(n: usize, cons: &[(i32, i32)]) -> Option<Vec<bool>> {
    let mut scc = SCC::new(2 * n);
    let ni = n as i32;
    for &(c1, c2) in cons.iter() {
        let x = if c1 > 0 {
            c1 - 1 + ni
        } else {
            -c1 - 1
        } as usize;
        let y = if c2 > 0 {
            c2 - 1
        } else {
            -c2 - 1 + ni
        } as usize;
        scc.add_edge(x, y);
        scc.add_edge((y + n) % (2 * n), (x + n) % (2 * n));
    }
    scc.scc();
    let mut result = vec![false; n];
    let top_ord = scc.top_order();
    for i in 0 .. n {
        if top_ord[i] == top_ord[i + n] {
            return None;
        }
        result[i] = top_ord[i] > top_ord[i + n];
    }
    Some(result)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, d: i64,
        xy: [(i64, i64); n],
    }
    let mut cons = vec![];
    for i in 0..n {
        let ii = i as i32 + 1;
        let (x1, y1) = xy[i];
        for j in 0..i {
            let jj = j as i32 + 1;
            let (x2, y2) = xy[j];
            if (x1 - x2).abs() < d {
                cons.push((ii, jj));
            }
            if (x1 - y2).abs() < d {
                cons.push((ii, -jj));
            }
            if (y1 - x2).abs() < d {
                cons.push((-ii, jj));
            }
            if (y1 - y2).abs() < d {
                cons.push((-ii, -jj));
            }
        }
    }
    let ans = two_sat(n, &cons);
    if let Some(ans) = ans {
        puts!("Yes\n");
        for i in 0..n {
            let c = if ans[i] {
                xy[i].1
            } else {
                xy[i].0
            };
            puts!("{}\n", c);
        }
    } else {
        puts!("No\n");
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
