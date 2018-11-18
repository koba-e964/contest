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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        k: usize,
        t: [i64; n],
    }
    let mut dp = vec![vec![0.0; k]; n + 1];
    for i in (0 .. n).rev() {
        let r = (t[i] % k as i64) as usize;
        let mut sum = 0.0;
        for j in 0 .. k { sum += dp[i + 1][j]; }
        let mut cursum = sum;
        for j in 0 .. k {
            let nxt = (j + r) % k;
            // dp[i][j] = (dp[i+1][j]+...+dp[i+1][k-1])/k + j/k^2*(dp[i+1][0]+...+dp[i+1][k-1])+...
            let mut tmp = cursum / k as f64;
            tmp += sum * j as f64 / ((k * k) as f64);
            tmp += (k - j) as f64 * (k - j - 1) as f64 / k as f64 / 2.0;
            tmp += ((k - j + k - j + k - 1) * k * j) as f64 / ((k * k) as f64) / 2.0;
            dp[i][j] = tmp;
            cursum -= dp[i + 1][nxt];
        }
    }
    let mut ans = dp[0][0];
    for t in t { ans += t as f64; }
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
