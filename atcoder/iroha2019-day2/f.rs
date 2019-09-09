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
        a: (usize, usize),
        b: (usize, usize),
        c: (usize, usize),
    }
    const W: usize = 11;
    let mut dp = vec![vec![vec![vec![vec![vec![-1.0 / 0.0; W]; W]; W]; W]; W]; W];
    dp[0][0][0][0][0][0] = 0.0;
    for i in 0..W {
        for j in 0..W {
            for k in 0..W {
                for l in 0..W {
                    for m in 0..W {
                        for n in 0..W {
                            if i + j + k + l + m + n == 0 {
                                continue;
                            }
                            let dist = [[i, j], [k, l], [m, n]];
                            let mut ma = -1.0 / 0.0;
                            for idx in 0..3 {
                                let mut dist = dist;
                                let mut tmp = 0.0;
                                if dist[idx][0] + dist[idx][1] == 0 {
                                    continue;
                                }
                                let prob = dist[idx][0] as f64
                                    / (dist[idx][0] + dist[idx][1]) as f64;
                                if dist[idx][0] > 0 {
                                    dist[idx][0] -= 1;
                                    let val = dp[dist[0][0]][dist[0][1]]
                                        [dist[1][0]][dist[1][1]]
                                        [dist[2][0]][dist[2][1]];
                                    tmp += prob * (100.0 - val);
                                    dist[idx][0] += 1;
                                }
                                if dist[idx][1] > 0 {
                                    dist[idx][1] -= 1;
                                    let val = dp[dist[0][0]][dist[0][1]]
                                        [dist[1][0]][dist[1][1]]
                                        [dist[2][0]][dist[2][1]];
                                    tmp += (1.0 - prob) * (50.0 - val);
                                    dist[idx][1] += 1;
                                }
                                if ma < tmp {
                                    ma = tmp;
                                }
                            }
                            dp[dist[0][0]][dist[0][1]]
                                [dist[1][0]][dist[1][1]]
                                [dist[2][0]][dist[2][1]] = ma;
                        }
                    }
                }
            }
        }
    }
    let val = dp[a.0][a.1][b.0][b.1][c.0][c.1];
    let coins = (2 * a.0 + a.1 + 2 * b.0 + b.1 + 2 * c.0 + c.1) as f64;
    puts!("{}\n", val / 2.0 + coins * 25.0);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
