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
        n: usize, k: usize,
        tc: [(i32, i64); n],
    }
    let mut dp = vec![vec![vec![vec![INF; k + 1]; k + 1]; k + 1]; n + 1];
    dp[0][0][0][0] = 0;
    for i in 0..n {
        for j in 0..k + 1 {
            for l in j..k + 1 {
                for m in l..k + 1 {
                    let val = dp[i][j][l][m];
                    let (dx, dy, dz) = match tc[i].0 {
                        3 => (1, 0, 0),
                        2 => (0, 1, 0),
                        1 => (0, 0, 1),
                        _ => unreachable!(),
                    };
                    dp[i + 1][j][l][m]
                        = min(dp[i + 1][j][l][m], val);
                    if j + dx > k || l + dy > k || m + dz > k {
                        continue;
                    }
                    dp[i + 1][j + dx][l + dy][m + dz]
                        = min(dp[i + 1][j + dx][l + dy][m + dz], val + tc[i].1);
                }
            }
        }
    }
    let mut ans = INF;
    for i in 0..k + 1 {
        for j in i..k + 1 {
            for l in j..k + 1 {
                if i + j + l == k {
                    ans = min(ans, dp[n][i][j][l]);
                }
            }
        }
    }
    if ans >= INF {
        puts!("Let's Shoot!\n");
    } else {
        puts!("{}\n", ans);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
