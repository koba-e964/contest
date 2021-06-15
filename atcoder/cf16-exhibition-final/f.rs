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
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

const INF: i64 = 1 << 60;

// Tags: dp
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        lr: [(i64, i64); n],
    }
    let mut lr = lr;
    lr.sort_by_key(|&(l, r)| -(l + r));
    if n % 2 == 0 {
        lr.push((0, 0));
    }
    let n = lr.len();
    let m = n / 2;
    let mut dp = vec![vec![[INF; 2]; m + 1]; m + 1];
    dp[0][0][0] = 0;
    for i in 0..m + 1 {
        for j in 0..m + 1 {
            for k in 0..2 {
                if i == m && j == m && k == 1 {
                    continue;
                }
                let (l, r) = lr[i + j + k];
                let val = dp[i][j][k];
                let len = l + r;
                if i < m {
                    dp[i + 1][j][k].chmin(val + i as i64 * len + r);
                }
                if j < m {
                    dp[i][j + 1][k].chmin(val + j as i64 * len + l);
                }
                if k == 0 {
                    dp[i][j][1].chmin(val + len * m as i64);
                }
            }
        }
    }
    let ans = dp[m][m][1];
    puts!("{}\n", ans);
}
