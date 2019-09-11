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

// The author read the editorial.
fn calc(a: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = a.len();
    let m = a[0].len();
    let mut perm = vec![vec![0; m]; n];
    // alloc
    let mut occ = vec![vec![false; m]; n];
    for i in 0..m {
        let mut graph = vec![vec![]; n]; 
        for j in 0..n {
            for k in 0..m {
                if !occ[j][k] {
                    graph[j].push(a[j][k]);
                }
            }
        }
        let (ans, matching) = hopcroft_karp(&graph, n);
        assert_eq!(ans, n);
        for j in 0..n {
            assert_ne!(matching[j], n);
            let mut idx = m;
            for k in 0..m {
                if !occ[j][k] && a[j][k] == matching[j] {
                    idx = k;
                    break;
                }
            }
            perm[j][idx] = i;
            occ[j][idx] = true;
        }
    }
    for i in 0..n {
        debugln!("{:?}", perm[i]);
    }
    perm
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize,
        a: [[usize; m]; n],
    }
    let mut trans = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            trans[i][j] = (a[i][j] - 1) / m;
        }
    }
    let perm = calc(&trans);
    let mut b = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            b[i][perm[i][j]] = a[i][j];
        }
        for j in 0..m {
            puts!("{} ", b[i][j]);
        }
        puts!("\n");
    }
    let mut c = vec![vec![0; m]; n];
    for j in 0..m {
        let mut col = vec![0; n];
        for i in 0..n {
            col[i] = b[i][j];
        }
        col.sort();
        for i in 0..n {
            c[i][j] = col[i];
        }
    }
    for i in 0..n {
        for j in 0..m {
            puts!("{} ", c[i][j]);
        }
        puts!("\n");
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
