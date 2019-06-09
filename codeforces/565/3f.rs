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
        n: usize,
        cds: [[(usize, i64)]; n],
    }
    let mut dp = vec![[-INF; 10]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let mut trans = [-INF; 4];
        let mut trans2 = [-INF; 4];
        dp[i + 1] = dp[i];

        let ref cd = cds[i];
        let mut wei = vec![vec![]; 4];
        for &(c, d) in cd {
            wei[c].push(d);
        }
        for c in 1..4 {
            wei[c].sort(); wei[c].reverse();
            wei[c] = wei[c][..min(wei[c].len(), if c == 1 { 3 } else { 1 })].to_vec();
        }
        let mut sum1 = 0;
        for a in 0..wei[1].len() + 1 {
            let mut sum2 = 0;
            for b in 0..wei[2].len() + 1 {
                let mut sum3 = 0;
                for c in 0..wei[3].len() + 1 {
                    if a + 2 * b + 3 * c <= 3 {
                        trans[a + b + c] = max(trans[a + b + c], sum1 + sum2 + sum3);
                        let mut ma = 0;
                        if a > 0 { ma = max(ma, wei[1][0]); }
                        if b > 0 { ma = max(ma, wei[2][0]); }
                        if c > 0 { ma = max(ma, wei[3][0]); }
                        trans2[a + b + c] = max(trans2[a + b + c],
                                                sum1 + sum2 + sum3 + ma);
                    }
                    if c < wei[3].len() { sum3 += wei[3][c]; }
                }
                if b < wei[2].len() { sum2 += wei[2][b]; }
            }
            if a < wei[1].len() { sum1 += wei[1][a]; }
        }

        for j in 0..10 {
            for k in 1..4 {
                dp[i + 1][(j + k) % 10] = max(dp[i + 1][(j + k) % 10],
                                              dp[i][j] + trans[k]);
                if j + k >= 10 {
                    dp[i + 1][(j + k) % 10] = max(dp[i + 1][(j + k) % 10],
                                                  dp[i][j] + trans2[k]);
                }
            }
        }
    }
    let ma: i64 = dp[n].iter().cloned().max().unwrap();
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
