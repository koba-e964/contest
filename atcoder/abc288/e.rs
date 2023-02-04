use std::cmp::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n],
        c: [i64; n],
        x: [usize1; m],
    }
    let mut y = vec![false; n];
    for x in x {
        y[x] = true;
    }
    const INF: i64 = 1 << 50;
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    for i in 0..n {
        let mut ep = vec![INF; n + 1];
        let mut inc = INF;
        for j in 0..min(n + 1, i + 2) {
            if j > 0 {
                // dp[j - 1] + a[i] + min(c[i - (j - 1)..=i])
                inc = min(inc, c[i - (j - 1)]);
                ep[j] = min(ep[j], dp[j - 1] + inc + a[i]);
            }
            if !y[i] {
                ep[j] = min(ep[j], dp[j]);
            }
        }
        dp = ep;
    }
    let &mi = dp.iter().min().unwrap();
    println!("{}", mi);
}
