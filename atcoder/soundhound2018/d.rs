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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        h: usize, w: usize,
        p: [[i64; w]; h],
        f: [[i64; w]; h],
    }
    let mut dp = vec![vec![-INF; w]; h + 1];
    dp[0][0] = 0;
    for i in 0..h {
        let mut ep = vec![vec![-INF; w]; 6];
        // -> <- ->
        for j in 0..w {
            ep[0][j] = dp[i][j] - f[i][j];
            if j > 0 {
                ep[0][j] = max(ep[0][j], ep[0][j - 1] - f[i][j]);
            }
        }
        for j in (0..w).rev() {
            ep[1][j] = ep[0][j] + p[i][j];
            if j + 1 < w {
                ep[1][j] = max(ep[1][j], ep[1][j + 1] + p[i][j] - f[i][j]);
            }
        }
        for j in 0..w {
            ep[2][j] = ep[1][j];
            if j > 0 {
                ep[2][j] = max(ep[2][j], ep[2][j - 1] - f[i][j]);
            }
        }
        for j in (0..w).rev() {
            ep[3][j] = dp[i][j] - f[i][j];
            if j + 1 < w {
                ep[3][j] = max(ep[3][j], ep[3][j + 1] - f[i][j]);
            }
        }
        for j in 0..w {
            ep[4][j] = ep[3][j] + p[i][j];
            if j > 0 {
                ep[4][j] = max(ep[4][j], ep[4][j - 1] + p[i][j] - f[i][j]);
            }
        }
        for j in (0..w).rev() {
            ep[5][j] = ep[4][j];
            if j + 1 < w {
                ep[5][j] = max(ep[5][j], ep[5][j + 1] - f[i][j]);
            }
        }
        for j in 0..w {
            dp[i + 1][j] = max(dp[i + 1][j], max(ep[2][j], ep[5][j]));
        }
    }
    for j in 0..w {
        puts!("{}\n", dp[h][j]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
