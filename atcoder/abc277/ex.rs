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
        for i in 0 .. n {
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
        for i in 0 .. n {
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

// 2-SAT solver.
// n: the number of variables (v_1, ..., v_n)
// Returns: None if there's no assignment that satisfies banned.
// Otherwise, it returns an assignment that safisfies banned. (true: true, false: false)
// Depends on: graph/SCC.rs
fn two_sat(n: usize, banned: &[((usize, bool), (usize, bool))]) -> Option<Vec<bool>> {
    let mut scc = SCC::new(2 * n);
    for &((c1, v1), (c2, v2)) in banned {
        let x = if !v1 {
            c1 + n
        } else {
            c1
        };
        let y = if !v2 {
            c2
        } else {
            c2 + n
        };
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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// Tags: scc, two-sat, partition-encoding
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize, q: usize,
        ablr: [(usize1, usize1, usize, usize); q],
    }
    let mut banned = vec![];
    // X[i] = j <=> y[i, j] = true
    // u[i, j] = false => k <= j => y[i, k] = false
    // y: FFTFFF...
    // u: FFTTTT...
    // mapping:
    // u[i, j]: 0 <= id < n * (m + 1)
    for (a, b, l, r) in ablr {
        for i in 0..m + 1 {
            if i > r {
                banned.push((
                    (a * (m + 1) + i - 1, false),
                    (a * (m + 1) + i - 1, false),
                ));
                continue;
            }
            if l > m + i {
                banned.push((
                    (a * (m + 1) + i, true),
                    (a * (m + 1) + i, true),
                ));
                continue;
            }
            if l > i {
                // X[a] <= i => X[b] > l - i - 1
                banned.push((
                    (a * (m + 1) + i, true),
                    (b * (m + 1) + l - i - 1, true),
                ));
            }
            if i + m > r {
                if i > 0 {
                    // X[a] > i - 1 => X[b] <= r - i
                    banned.push((
                        (a * (m + 1) + i - 1, false),
                        (b * (m + 1) + r - i, false),
                    ));
                } else {
                    // X[b] <= r
                    banned.push((
                        (b * (m + 1) + r, false),
                        (b * (m + 1) + r, false),
                    ));
                }
            }
        }
    }
    for i in 0..n {
        // u[i, m] = true
        banned.push((
            (i * (m + 1) + m, false),
            (i * (m + 1) + m, false),
        ));
        for j in 0..m + 1 {
            // u[i, j] = false => u[i, j - 1] => false
            if j > 0 {
                banned.push((
                    (i * (m + 1) + j, false),
                    (i * (m + 1) + j - 1, true),
                ));
            }
        }
    }
    if let Some(sol) = two_sat(n * (m + 1), &banned) {
        let mut x = vec![0; n];
        for i in 0..n {
            for j in 0..m + 1 {
                if sol[i * (m + 1) + j] {
                    x[i] = j;
                    break;
                }
            }
        }
        putvec!(x);
    } else {
        puts!("-1\n");
    }
}
