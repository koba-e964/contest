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
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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
    input!(n: i64);
    const N: usize = 30;
    // dp[pos][delta][eq][zero]
    let mut dp = vec![vec![[[0i64; 2]; 2]; 4 * N]; N + 1];
    dp[0][2 * N][1][1] += 1;
    let mut dig = vec![0; N];
    let mut v = n;
    for i in 0..N {
        let mut r = v % 5;
        if r >= 3 { r -= 5; }
        dig[N - 1 - i] = r;
        v = (v - r) / 5;
    }
    for i in 0..N {
        for j in 0..4 * N {
            for eq in 0..2 {
                for zero in 0..2 {
                    let val = dp[i][j][eq][zero];
                    if val == 0 { continue; }
                    for delta in -2..3 {
                        if delta < 0 && zero == 1 {
                            continue;
                        }
                        if eq == 1 && delta > dig[i] {
                            continue;
                        }
                        let to = j.wrapping_add(delta as usize);
                        if to >= 4 * N { continue; }
                        dp[i + 1][to][eq & if delta == dig[i] { 1 } else { 0 }]
                            [zero & if delta == 0 { 1 } else { 0 }] += val;
                    }
                }
            }
        }
    }
    let mut tot: i64 = 0;
    for eq in 0..2 {
        tot += dp[N][2 * N][eq][0];
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
