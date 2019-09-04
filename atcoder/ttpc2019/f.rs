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

const INF: i64 = 1 << 50;

fn calc<I: Iterator<Item=usize> + Clone>(
    revg: &[Vec<(usize, i64)>],
    dp: &mut [i64],
    ep: &[usize],
    iter: I) {
    let n = revg.len();
    let mut pre = vec![vec![INF; n]; 2];
    for i in 0..2 {
        pre[i][ep[i]] = 0;
        for v in iter.clone() {
            for &(w, c) in &revg[v] {
                pre[i][v] = min(pre[i][v], pre[i][w] + c);
            }
        }
    }
    for v in iter {
        dp[v] = min(dp[v], pre[0][v] + pre[1][v]);
        for &(w, c) in &revg[v] {
            dp[v] = min(dp[v], dp[w] + c);
        }
    }
}

fn calc_direct(
    revg: &[Vec<(usize, i64)>],
    a: usize, b: usize) -> i64 {
    let n = revg.len();
    let mut dp = vec![INF; n];
    dp[a] = 0;
    for v in 0..n {
        for &(w, c) in &revg[v] {
                dp[v] = min(dp[v], dp[w] + c);
        }
    }
    dp[b]
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize,
        ep: [usize1; 4],
        cst: [(i64, usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    let mut revg = vec![vec![]; n];
    for &(c, s, t) in &cst {
        g[s].push((t, c));
        revg[t].push((s, c));
    }
    let mut dp0 = vec![INF; n];
    let mut dp1 = vec![INF; n];
    calc(&revg, &mut dp0, &[ep[0], ep[2]], 0..n);
    calc(&g, &mut dp1, &[ep[1], ep[3]], (0..n).rev());
    let mut mi = INF;
    for i in 0..n {
        mi = min(mi, dp0[i] + dp1[i]);
    }
    mi = min(mi,
             calc_direct(&revg, ep[0], ep[1])
             + calc_direct(&revg, ep[2], ep[3]));
    if mi >= INF {
        puts!("Impossible\n");
    } else {
        puts!("{}\n", mi);
    }
    
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
