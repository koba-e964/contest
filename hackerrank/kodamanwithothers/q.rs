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

const INF: i64 = 1 << 50;

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize, x: usize, k: i64,
        p: [i64; x],
        abc: [(usize1, i64, usize1); n],
    }
    let mut k = k;
    let mut teacher = vec![vec![]; m];
    for &(a, b, c) in &abc {
        let diff = p[a] - b;
        if diff > 0 {
            teacher[c].push(diff);
        } else {
            k -= 1;
        }
    }
    for i in 0..m {
        teacher[i].sort();
    }
    let mut dp = vec![vec![INF; n + 1]; m + 1];
    dp[0][0] = 0;
    for i in 0..m {
        for (idx, &b) in teacher[i].iter().enumerate() {
            for j in 0..n - idx {
                dp[i + 1][j + 1 + idx] =
                    min(dp[i + 1][j + 1 + idx], dp[i][j] + b);
            }
        }
        for j in 0..n + 1 {
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
        }
    }
    // debugln!("dp = {:?}", dp);
    let mut mi = INF;
    for i in 0..n + 1 {
        if i as i64 >= k {
            mi = min(mi, dp[m][i]);
        }
    }
    puts!("{}\n", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
