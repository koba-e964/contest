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

/*
 * Dijkstra's algorithm.
 * Verified by: AtCoder ABC164 (https://atcoder.jp/contests/abc164/submissions/12423853)
 */

struct Dijkstra {
    edges: Vec<Vec<(usize, i64)>>, // adjacent list representation
}

impl Dijkstra {
    fn new(n: usize) -> Self {
        Dijkstra { edges: vec![Vec::new(); n] }
    }
    fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
        self.edges[from].push((to, cost));
    }
    /*
     * This function returns a Vec consisting of the distances from vertex source.
     */
    fn solve(&self, source: usize, inf: i64) -> Vec<i64> {
        let n = self.edges.len();
        let mut d = vec![inf; n];
        // que holds (-distance, vertex), so that que.pop() returns the nearest element.
        let mut que = std::collections::BinaryHeap::new();
        que.push((0, source));
        while let Some((cost, pos)) = que.pop() {
            let cost = -cost;
            if d[pos] <= cost {
                continue;
            }
            d[pos] = cost;
            for &(w, c) in &self.edges[pos] {
                let newcost = cost + c;
                if d[w] > newcost {
                    d[w] = newcost + 1;
                    que.push((-newcost, w));
                }
            }
        }
        return d;
    }
}

fn is_pal(a: &[char]) -> bool {
    let n = a.len();
    for i in 0..n / 2 {
        if a[i] != a[n - 1 - i] {
            return false;
        }
    }
    true
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        sc: [(chars, i64); n],
    }
    let mut srevc = vec![];
    let mut st = vec![];
    let mut strev = vec![];
    st.push(vec![]);
    strev.push(vec![]);
    for &(ref s, c) in &sc {
        let l = s.len();
        for i in 0..l {
            st.push(s[i..].to_vec());
        }
        let mut t = s.clone();
        t.reverse();
        for i in 0..l {
            strev.push(t[i..].to_vec());
        }
        srevc.push((t, c));
    }
    st.sort(); st.dedup();
    strev.sort(); strev.dedup();
    let m1 = st.len();
    let m2 = strev.len();
    let mut dijk = Dijkstra::new(m1 + m2 + 1);
    for i in 0..m1 {
        let s = &st[i];
        for &(ref t, c) in &srevc {
            let l = min(s.len(), t.len());
            if s[..l] != t[..l] {
                continue;
            }
            if s.len() > l {
                // e.g. s = "abcd", t = "abc"
                let nxt = s[l..].to_vec();
                let idx = st.binary_search(&nxt).unwrap();
                dijk.add_edge(i, idx, c);
            } else {
                // e.g. s = "abc", t = "abcde"
                let nxt = t[l..].to_vec();
                let idx = strev.binary_search(&nxt).unwrap();
                dijk.add_edge(i, m1 + idx, c);
            }
        }
    }
    for i in 0..m2 {
        let s = &strev[i];
        for &(ref t, c) in &sc {
            let l = min(s.len(), t.len());
            if s[..l] != t[..l] {
                continue;
            }
            if s.len() > l {
                // e.g. s = "abcd", t = "abc"
                let nxt = s[l..].to_vec();
                let idx = strev.binary_search(&nxt).unwrap();
                dijk.add_edge(m1 + i, m1 + idx, c);
            } else {
                // e.g. s = "abc", t = "abcde"
                let nxt = t[l..].to_vec();
                let idx = st.binary_search(&nxt).unwrap();
                dijk.add_edge(m1 + i, idx, c);
            }
        }
    }
    for &(ref s, c) in &sc {
        let idx = st.binary_search(s).unwrap();
        dijk.add_edge(m1 + m2, idx, c);
    }
    for &(ref t, c) in &srevc {
        let idx = strev.binary_search(t).unwrap();
        dijk.add_edge(m1 + m2, m1 + idx, c);
    }
    const INF: i64 = 1 << 55;
    let sol = dijk.solve(m1 + m2, INF);
    let mut mi = INF;
    for i in 0..m1 {
        if is_pal(&st[i]) {
            mi = min(mi, sol[i]);
        }
    }
    for i in 0..m2 {
        if is_pal(&strev[i]) {
            mi = min(mi, sol[m1 + i]);
        }
    }
    puts!("{}\n", if mi >= INF { -1 } else { mi });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
