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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        h: usize, w: usize,
        c: [chars; h],
    }
    let mut reach = vec![vec![false; h]; h];
    for i in 0..h {
        let mut dp = vec![vec![false; w]; h];
        if c[i][0] == '#' { continue; }
        dp[i][0] = true;
        for j in i..h {
            for k in 0..w {
                if !dp[j][k] { continue; }
                if j + 1 < h && c[j + 1][k] == '.' {
                    dp[j + 1][k] = true;
                }
                if k + 1 < w && c[j][k + 1] == '.' {
                    dp[j][k + 1] = true;
                }
            }
        }
        for j in 0..h {
            reach[i][j] = dp[j][w - 1];
        }
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![vec![INF; h]; h];
    for i in 0..h {
        for j in 0..h {
            if reach[i][j] {
                dist[i][j] = 0;
            }
        }
    }
    for k in 0..h {
        for i in 0..h {
            for j in 0..h {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }
    let ans = dist[0][h - 1] == 0;
    let mut ok = false;
    for i in 0..h {
        if reach[i][i] {
            ok = true;
        }
    }
    puts!("{}\n", if ans && ok { "Yay!" } else { ":(" });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
