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

fn max(a: f64, b: f64) -> f64 {
    if a < b {
        b
    } else {
        a
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, k: usize,
        me: f64,
        abcd: [(f64, f64, usize, usize); n],
    }
    let mut pass = 0.0;
    let mut fail = 2e12;
    const INF: f64 = 1e40;
    
    for _ in 0..100 {
        let mid = (pass + fail) / 2.0;
        let mut dp = vec![vec![-INF; k + 1]; n + 1];
        dp[0][0] = me - mid;
        for (i, &(a, b, c, d)) in abcd.iter().enumerate() {
            for j in 0..k + 1 {
                dp[i + 1][j] = max(dp[i + 1][j], dp[i][j] + a - mid);
            }
            for j in 0..k + 1 - c {
                dp[i + 1][j + c] = max(dp[i + 1][j + c], dp[i + 1][j] + b);
            }
            for j in 0..k + 1 - d {
                dp[i + 1][j + d] = max(dp[i + 1][j + d], dp[i][j]);
            }
        }
        if dp[n].iter().any(|&x| x >= 0.0) {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    puts!("{}\n", pass);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
