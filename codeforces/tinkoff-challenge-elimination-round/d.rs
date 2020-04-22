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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, k: usize,
        m: usize,
        uvc: [(usize1, usize1, i32); m],
    }
    let k = k - 1;
    const INF: i32 = 1 << 29;
    // dp[i][j][...]: can visit [i, j) next time
    let mut dp = vec![vec![vec![[INF; 2]; k + 1]; n + 1]; n + 1];
    for i in 0..n {
        dp[0][i][0][1] = 0;
        dp[i + 1][n][0][0] = 0;
    }
    let mut g = vec![vec![]; n];
    for &(u, v, c) in &uvc {
        g[u].push((v, c));
    }
    for l in 0..k {
        for i in 0..n {
            for j in i + 1..n + 1 {
                for d in 0..2 {
                    if dp[i][j][l][d] >= INF { continue; }
                    let mut dsts = vec![];
                    if i > 0 && d == 0 {
                        for &(w, c) in &g[i - 1] {
                            if i <= w && w < j {
                                dsts.push((w, c));
                            }
                        }
                    }
                    if j < n && d == 1 {
                        for &(w, c) in &g[j] {
                            if i <= w && w < j {
                                dsts.push((w, c));
                            }
                        }
                    }
                    for (w, c) in dsts {
                        dp[i][w][l + 1][1] = min(dp[i][w][l + 1][1],
                                              dp[i][j][l][d] + c);
                        dp[w + 1][j][l + 1][0] = min(dp[w + 1][j][l + 1][0],
                                                  dp[i][j][l][d] + c);
                    }
                }
            }
        }
    }
    let mut mi = INF;
    for i in 0..n + 1 {
        for j in i..n + 1 {
            for d in 0..2 {
                mi = min(mi, dp[i][j][k][d]);
            }
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
