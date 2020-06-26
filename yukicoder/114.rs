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

/// Generates an Iterator over subsets of univ, in the descending order. 
/// Verified by: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3050308
struct SubsetIter { bits: Option<usize>, univ: usize }
impl Iterator for SubsetIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self.bits {
            None => None,
            Some(bits) => {
                let ans = bits;
                self.bits =
                    if bits == 0 { None }
                else { Some((bits - 1) & self.univ) };
                Some(ans)
            }
        }
    }
}
fn subsets(univ: usize) -> SubsetIter {
    SubsetIter { bits: Some(univ), univ: univ }
}

/**
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
 */
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![1; n] }
    }
    fn root(&mut self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        self.disj[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.disj[x] = y;
        self.rank[y] += self.rank[x];
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.rank[x]
    }
}

// Tags: steiner-tree
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, m: usize, t: usize,
        abc: [(usize1, usize1, i64); m],
        v: [usize1; t],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, c) in &abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    const INF: i64 = 1 << 50;
    if t <= 15 {
        // O(3^t * poly(n, m))
        let mut dp = vec![vec![INF; 1 << t]; n];
        for i in 0..t {
            dp[v[i]][1 << i] = 0;
        }
        // Reuse que for performance
        let mut que = BinaryHeap::new();
        for bits in 1usize..1 << t {
            for sub in subsets(bits) {
                if sub != 0 && sub != bits {
                    for i in 0..n {
                        dp[i][bits] = min(dp[i][bits],
                                          dp[i][sub] + dp[i][bits - sub]);
                    }
                        
                }
            }
            assert!(que.is_empty());
            for i in 0..n {
                que.push((-dp[i][bits], i));
            }
            while let Some((d, v)) = que.pop() {
                let d = -d;
                for &(w, c) in &g[v] {
                    if d + c < dp[w][bits] {
                        dp[w][bits] = d + c;
                        que.push((-(d + c), w));
                    }
                }
            }
        }
        let mut mi = INF;
        for i in 0..n {
            mi = min(mi, dp[i][(1 << t) - 1]);
        }
        puts!("{}\n", mi);
    } else {
        let mut has = vec![true; n];
        for &v in &v {
            has[v] = false;
        }
        let rem: Vec<_> = (0..n).filter(|&i| has[i]).collect();
        assert_eq!(rem.len(), n - t);
        let mut mi = INF;
        let mut edges = vec![];
        for a in 0..n {
            for &(b, c) in &g[a] {
                if a < b {
                    edges.push((c, a, b));
                }
            }
        }
        edges.sort();
        for bits in 0..1 << (n - t) {
            let mut app = vec![false; n];
            let mut nv = 0;
            for &v in &v {
                app[v] = true;
                nv += 1;
            }
            for i in 0..n - t {
                if (bits & 1 << i) != 0 {
                    app[rem[i]] = true;
                    nv += 1;
                }
            }
            let mut uf = UnionFind::new(n);
            let mut tot: i64 = 0;
            let mut conn = nv;
            for &(c, a, b) in &edges {
                if !app[a] || !app[b] { continue; }
                if uf.is_same_set(a, b) {
                    continue;
                }
                uf.unite(a, b);
                tot += c;
                conn -= 1;
                // pruning
                if conn == 1 {
                    break;
                }
                if tot >= mi {
                    break;
                }
            }
            if conn == 1 {
                mi = min(mi, tot);
            }
        }
        puts!("{}\n", mi);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
