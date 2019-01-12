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

const INF: i64 = 1 << 50;

fn dfs(v: usize, par: usize, g: &[Vec<usize>], a: &[i64],
       dp: &mut [Vec<i64>], ep: &mut [Vec<i64>]) -> usize {
    let n = g.len();
    let mut sz = 1;
    // init
    if a[v] < 0 {
        dp[v][0] = a[v];
    } else {
        ep[v][0] = a[v]; // power source only
    }
    for &w in g[v].iter() {
        if par == w { continue; }
        let mut dp2 = vec![INF; n + 1];
        let mut ep2 = vec![INF; n + 1];
        let sub = dfs(w, v, g, a, dp, ep);
        for i in 0 .. sz + 1 {
            if dp[v][i] > INF / 2 { continue; }
            for j in 0 .. sub + 1 {
                if dp[w][j] < INF / 2 {
                    dp2[i + j] = min(dp2[i + j], dp[v][i] + dp[w][j]);
                    if dp[w][j] < 0 {
                        dp2[i + j + 1] = min(dp2[i + j + 1], dp[v][i]);
                    }
                }
                if ep[w][j] < INF / 2 {
                    dp2[i + j] = min(dp2[i + j], dp[v][i] + ep[w][j]);
                        dp2[i + j + 1] = min(dp2[i + j + 1], dp[v][i]);
                }
            }
        }
        for i in 0 .. sz + 1 {
            if ep[v][i] > INF / 2 { continue; }
            for j in 0 .. sub + 1 {
                if dp[w][j] < INF / 2 {
                    dp2[i + j] = min(dp2[i + j], ep[v][i] + dp[w][j]);
                    if dp[w][j] < 0 {
                        ep2[i + j + 1] = min(ep2[i + j + 1], ep[v][i]);
                    }
                }
                if ep[w][j] < INF / 2 {
                    ep2[i + j] = min(ep2[i + j], ep[v][i] + ep[w][j]);
                    ep2[i + j + 1] = min(ep2[i + j + 1], ep[v][i]);
                }
            }
        }
        dp[v] = dp2;
        ep[v] = ep2;
        sz += sub;
    }
    // eprintln!("v = {}, dp = {:?}, ep = {:?}", v, dp[v], ep[v]);
    sz
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        a: [i64; n],
        xy: [(usize1, usize1); n - 1],
    }
    let mut g = vec![Vec::new(); n];
    for (x, y) in xy {
        g[x].push(y);
        g[y].push(x);
    }
    let mut dp = vec![vec![INF; n + 1]; n];
    let mut ep = vec![vec![INF; n + 1]; n];
    dfs(0, n, &g, &a, &mut dp, &mut ep);
    let mut mi = n;
    for i in 0 .. n + 1 {
        if dp[0][i] < 0 || ep[0][i] < INF / 2 {
            mi = i;
            break;
        }
    }
    puts!("{}\n", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
