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
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        h: usize, w: usize,
        a: [[i32; w]; h],
        b: [[i32; w]; h],
    }
    const BIAS: usize = 13_000;
    let mut dp = vec![vec![vec![false; 2 * BIAS]; w]; h];
    dp[0][0][BIAS] = true;
    for i in 0..h {
        for j in 0..w {
            for k in 0..2 * BIAS {
                if !dp[i][j][k] { continue; }
                let diff = (a[i][j] - b[i][j]).abs() as usize;
                if i + 1 < h {
                    // (+1, +0)
                    if k >= diff {
                        dp[i + 1][j][k - diff] = true;
                    }
                    if k + diff < 2 * BIAS {
                        dp[i + 1][j][k + diff] = true;
                    }
                }
                if j + 1 < w {
                    // (+0, +1)
                    if k >= diff {
                        dp[i][j + 1][k - diff] = true;
                    }
                    if k + diff < 2 * BIAS {
                        dp[i][j + 1][k + diff] = true;
                    }
                }
            }
        }
    }
    let mut ma = 2 * BIAS as i32;
    let diff = (a[h - 1][w - 1] - b[h - 1][w - 1]).abs();
    for i in 0..2 * BIAS {
        if !dp[h - 1][w - 1][i] { continue; }
        ma = min(ma, (i as i32 - diff - BIAS as i32).abs());
        ma = min(ma, (i as i32 + diff - BIAS as i32).abs());
    }
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
