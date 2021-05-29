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

// Tags: dp, n^2
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        ab: [[i64; 2]; n],
    }
    const INF: i64 = 1 << 50;
    let mut dp = vec![[INF; 2]; n + 1];
    let mut acc = vec![[0; 2]; n + 1];
    let mut acc_m = vec![[0; 2]; n + 1];
    for i in 0..n {
        for j in 0..2 {
            acc[i + 1][j] = acc[i][j] + ab[i][j];
            acc_m[i + 1][j] = acc_m[i][j] + ab[i][j] * i as i64;
        }
    }
    let get_incr = |l: usize, r: usize, j| {
        (acc_m[r][1 - j] - acc_m[l][1 - j]) - (acc[r][1 - j] - acc[l][1 - j]) * (l as i64 - 1)
    };
    let get_decr = |l: usize, r: usize, j| {
        (acc[r][1 - j] - acc[l][1 - j]) * r as i64 - (acc_m[r][1 - j] - acc_m[l][1 - j])
    };
    for i in 1..n + 1 {
        for j in 0..2 {
            if i < n {
                // init
                dp[i][j] = get_decr(0, i, j);
                // cont
                for k in 1..i {
                    let mid = (i + k) / 2;
                    let val = dp[k][1 - j] + get_decr(mid, i, j) + get_incr(k, mid, j);
                    dp[i][j].chmin(val);
                }
            }
            // end
            if i == n {
                for k in 1..n {
                    let val = dp[k][1 - j] + get_incr(k, n, j);
                    dp[i][j].chmin(val);
                }
            }
        }
    }
    puts!("{}\n", min(dp[n][0], dp[n][1]));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
