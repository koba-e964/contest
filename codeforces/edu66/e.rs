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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize,
        lr: [(usize, usize); n],
        xy: [(usize, usize); m],
    }
    const W: usize = 500100;
    let mut rgt = vec![0; W];
    for i in 0..W {
        rgt[i] = i;
    }
    for &(l, r) in &lr {
        rgt[l] = max(rgt[l], r);
    }
    const B: usize = 18;
    let mut dp = vec![vec![0; W]; B];
    for i in 0..W {
        dp[0][i] = if i > 0 {
            max(rgt[i], dp[0][i - 1])
        } else {
            rgt[i]
        };
    }
    for i in 1..B {
        for j in 0..W {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }
    for &(mut x, y) in &xy {
        let mut cnt = 0;
        for i in (0..B).rev() {
            if dp[i][x] < y {
                x = dp[i][x];
                cnt |= 1 << i;
            }
        }
        if dp[0][x] < y {
            puts!("-1\n");
        } else {
            puts!("{}\n", cnt + 1);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
