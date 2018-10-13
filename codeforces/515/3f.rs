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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

const INF: i64 = 1 << 60;

fn dist((a, b): (i64, i64), (c, d): (i64, i64)) -> i64 {
    (a - c).abs() + (b - d).abs()
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut s = Vec::new();
    for &(x, y) in xy.iter() {
        s.push(max(x, y));
    }
    s.sort();
    s.dedup();
    let m = s.len();
    let mut aa = vec![Vec::new(); m + 1];
    for &(x, y) in xy.iter() {
        let idx = s.binary_search(&max(x, y)).unwrap();
        aa[idx + 1].push((x, y));
    }
    aa[0].push((0, 0));
    for i in 0 .. m + 1 {
        for u in aa[i].iter_mut() { u.1 *= -1; }
        aa[i].sort();
        for u in aa[i].iter_mut() { u.1 *= -1; }
        if aa[i].len() == 1 {
            let v = aa[i][0];
            aa[i].push(v);
        }
        if aa[i].len() >= 2 {
            let v = aa[i].pop().unwrap();
            while aa[i].len() > 1 {
                aa[i].pop();
            }
            aa[i].push(v);
        }
    }
    let mut dp = vec![[INF; 2]; m + 1];
    dp[0][0] = 0;
    dp[0][1] = 0;
    for i in 0 .. m {
        for j in 0 .. 2 {
            for k in 0 .. 2 {
                dp[i + 1][1 - k] = min(dp[i + 1][1 - k], dp[i][j] + dist(aa[i][j], aa[i + 1][k]) + dist(aa[i + 1][k], aa[i + 1][1 - k]));
            }
        }
    }
    puts!("{}\n", min(dp[m][0], dp[m][1]));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
