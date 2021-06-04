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

/*
 * Online monotone minima dp. For example, monge dp can be efficiently computed
 * by online_dc.
 * Verified by: https://yukicoder.me/problems/no/705
 * submission: https://yukicoder.me/submissions/566775
 */
const INF: i64 = 1 << 60;

// Complexity: O(n log m + m) where n = r - l, m = b - a.
fn monotone_minima<F>(l: usize, r: usize, a: usize, b: usize,
                      frm: &[i64], lat: &mut [i64],
                      cost_fun: &F)
where F: Fn(usize, usize) -> i64 {
    let n = r - l;
    let m = b - a;
    if n == 0 || m == 0 {
        return;
    }
    let mid = (a + b) / 2;
    let mut mi = (INF, n);
    for i in l..r {
        let cost = cost_fun(i, mid);
        mi = std::cmp::min(mi, (cost + frm[i], i));
    }
    let idx = mi.1;
    assert!(l <= idx && idx < r);
    lat[mid] = std::cmp::min(lat[mid], mi.0);
    monotone_minima(l, idx + 1, a, mid, frm, lat, cost_fun);
    monotone_minima(idx, r, mid + 1, b, frm, lat, cost_fun);
}

// Tags: monotone-minima, divide-and-conquer
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        x: [i64; n],
    }
    let mut x = x;
    x.sort();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + x[i];
    }
    const INF: i64 = 1 << 50;
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    let cost_fun = |k, l| {
        let mid = (k + l) / 2;
        acc[l] + acc[k] - 2 * acc[mid] - (l + k - 2 * mid) as i64 * x[mid]
    };
    for _ in 0..m {
        let mut ep = vec![INF; n + 1];
        monotone_minima(0, n, 0, n + 1, &dp, &mut ep, &cost_fun);
        dp = ep;
    }
    puts!("{}\n", dp[n]);
}
