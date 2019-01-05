#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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
type Idx = i32;
const INF: Idx = 1 << 20;

fn dfs<F>(g: &[Vec<(Idx, usize)>], lt: &F,
          conn: &mut [Idx], v: usize, cur: Idx)
where F: Fn(Idx, Idx) -> bool {
    if !lt(cur, conn[v]) { return; }
    conn[v] = cur;
    for &(_, w) in g[v].iter() {
        dfs(g, lt, conn, w, cur);
    }
}

// conn[i][j] = k: i -> j is reachable if you use edges from i with index <= k
// and edges that are not from i.
fn traverse<F>(g: &[Vec<(Idx, usize)>], lt: F, min: Idx) -> Vec<Vec<Idx>>
where F: Fn(Idx, Idx) -> bool {
    let n = g.len();
    let mut conn = vec![vec![-min; n]; n];
    for i in 0 .. n {
        conn[i][i] = min;
        for &(idx, w) in g[i].iter() {
            dfs(&g, &lt, &mut conn[i], w, idx);
        }
    }
    conn
}


fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        ab: [(usize1, usize1)],
    }
    let mut g = vec![Vec::new(); n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        g[a].push((i as Idx, b));
    }
    // g[i] is already in the increasing order
    let conn_orig = traverse(&g, |x, y| x < y, -INF);
    // decreasing order
    for adj in g.iter_mut() {
        adj.reverse();
    }
    let conn_rev = traverse(&g, |x, y| x > y, INF);
    for (i, (a, b)) in ab.into_iter().enumerate() {
        let i = i as Idx;
        let orig = conn_orig[a][b] != i || conn_rev[a][b] != i;
        let rev = conn_orig[b][a] < INF;
        puts!("{}\n", if orig ^ rev { "diff" } else { "same" });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
