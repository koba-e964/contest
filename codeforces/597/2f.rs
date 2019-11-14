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

fn calc(x: i64, y: i64) -> i64 {
    let n = 30;
    let mut dp = vec![[[0i64; 2]; 2]; n + 1];
    dp[0][0][0] = 1;
    for i in 0..n {
        for j in 0..2 {
            for k in 0..2 - j {
                for a in 0..2 {
                    let xlim = (x >> i) - a as i64;
                    if xlim < j as i64 { continue; }
                    let na = x >> (i + 1) > (xlim - j as i64) / 2;
                    let na = if na { 1 } else { 0 };
                    for b in 0..2 {
                        let ylim = (y >> i) - b as i64;
                        if ylim < k as i64 { continue; }
                        let nb = y >> (i + 1) > (ylim - k as i64) / 2;
                        let nb = if nb { 1 } else { 0 };
                        dp[i + 1][na][nb] += dp[i][a][b];
                    }
                }
            }
        }
    }
    let mut tot = 0;
    for i in 0..2 {
        for j in 0..2 {
            tot += dp[n][i][j];
        }
    }
    tot
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        lr: [(i64, i64)],
    }
    debugln!("{}", calc(1, 4));
    for (l, r) in lr {
        puts!("{}\n", calc(r, r) + calc(l - 1, l - 1) - 2 * calc(l - 1, r));
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
