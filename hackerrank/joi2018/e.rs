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
        m: usize,
        a: [usize; n],
    }
    let tbl = [1000, 0, 1, 3, 6, 10];
    let mut dp = vec![vec![0; m + 1]; n + 1];
    let mut prev = vec![[n + 1; 15]; n + 1];
    for i in 1 .. n + 1 {
        let mut mi = 10;
        let mut ma = 0;
        for j in (0 .. i).rev() {
            mi = min(mi, a[j]);
            ma = max(ma, a[j]);
            let idx = tbl[ma] + mi - 1;
            if prev[i][idx] > n {
                prev[i][idx] = j;
            }
        }
    }
    for i in 0 .. m {
        for j in 1 .. n + 1 {
            dp[j][i + 1] = max(dp[j][i + 1], dp[j][i]);
            for mi in 1 .. 6 {
                for ma in mi .. 6 {
                    let idx = tbl[ma] + mi - 1;
                    let pre = prev[j][idx];
                    if pre <= n {
                        dp[j][i + 1] = max(dp[j][i + 1], dp[pre][i] + (ma - mi));
                    }
                }
            }
        }
    }
    puts!("{}\n", dp[n][m]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
