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
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

// https://en.wikipedia.org/wiki/Hopcroft–Karp_algorithm#Pseudocode
fn hopcroft_karp(g: &[Vec<usize>], m: usize)
                 -> (usize, Vec<usize>) {
    let n = g.len();
    let mut pu = vec![m; n + 1]; // n: NIL
    let mut pv = vec![n; m + 1]; // m: NIL
    let mut dist = vec![0; n + 1];

    let mut matching = 0;

    fn bfs(g: &[Vec<usize>], pu: &mut [usize], pv: &mut [usize],
           dist: &mut [usize]) -> bool {
        let n = g.len();
        let m = pv.len() - 1;
        let mut que = std::collections::VecDeque::new();
        for i in 0..n {
            if pu[i] == m {
                dist[i] = 0;
                que.push_back(i);
            } else {
                dist[i] = usize::max_value();
            }
        }
        dist[n] = usize::max_value();
        while let Some(u) = que.pop_front() {
            if dist[u] < dist[n] {
                for &v in &g[u] {
                    if dist[pv[v]] == usize::max_value() {
                        dist[pv[v]] = dist[u] + 1;
                        que.push_back(pv[v]);
                    }
                }
            }
        }
        dist[n] != usize::max_value()
    }

    fn dfs(g: &[Vec<usize>], pu: &mut [usize], pv: &mut [usize],
           dist: &mut [usize],
           u: usize) -> bool {
        let n = g.len();
        if u != n {
            for &v in &g[u] {
                if dist[pv[v]] == dist[u] + 1 {
                    let pvv = pv[v];
                    if dfs(g, pu, pv, dist, pvv) {
                        pv[v] = u;
                        pu[u] = v;
                        return true;
                    }
                }
            }
            dist[u] = usize::max_value();
            return false;
        }
        true
    }

    while bfs(g, &mut pu, &mut pv, &mut dist) {
        for u in 0..n {
            if pu[u] == m {
                if dfs(g, &mut pu, &mut pv, &mut dist, u) {
                    matching += 1;
                }
            }
        }
    }
    (matching, pu)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        abc: [[i64; 3]],
    }
    let mut used = vec![];
    let mut edges = vec![];
    for x in &abc {
        let prod = x[0] * x[1] * x[2];
        for j in 0..3 {
            for k in 1..x[j] {
                let p = prod / x[j] * k;
                let q = prod - p;
                used.push(p);
                used.push(q);
                edges.push((p, q));
            }
        }
    }
    used.sort();
    used.dedup();
    let m = used.len();
    let mut g = vec![vec![]; m];
    for &(x, y) in &edges {
        let x = used.binary_search(&x).unwrap();
        let y = used.binary_search(&y).unwrap();
        g[x].push(y);
        g[y].push(x);
    }
    puts!("{}\n", 2 * m - hopcroft_karp(&g, m).0);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
