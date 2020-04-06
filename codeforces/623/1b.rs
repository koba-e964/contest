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
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, k: usize,
        a: [usize1; k],
    }
    const INF: i32 = 1 << 29;
    let mut b = vec![false; 1 << n];
    for &a in &a { b[a] = true; }
    let mut dp = vec![vec![]; n + 1];
    dp[1] = vec![[[-INF; 2]; 2]; 1 << (n - 1)];
    for i in 0..1 << (n - 1) {
        let c = i32::from(b[2 * i]) + i32::from(b[2 * i + 1]);
        match c {
            0 => 
                dp[1][i][0][0] = 0,
            1 => {
                dp[1][i][0][1] = 1;
                dp[1][i][1][0] = 1;
            }
            2 =>
                dp[1][i][1][1] = 1,
            _ => panic!(),
        }
    }
    for i in 1..n {
        dp[i + 1] = vec![[[-INF; 2]; 2]; 1 << (n - i - 1)];
        for bits in 0..1 << (n - i - 1) {
            for a in 0..2 {
                for b in 0..2 {
                    let val0 = dp[i][2 * bits][a][b];
                    if val0 <= -INF { continue; }
                    for c in 0..2 {
                        for d in 0..2 {
                            let val1 = dp[i][2 * bits + 1][c][d];
                            if val1 <= -INF { continue; }
                            let up = a | c;
                            let down = b | d;
                            for &(e, f) in &[
                                (a, c | down),
                                (c, a | down),
                            ] {
                                let delta = (up + down + f) as i32;
                                dp[i + 1][bits][e][f] = max(
                                    dp[i + 1][bits][e][f],
                                    val0 + val1 + delta);
                            }
                        }
                    }
                }
            }
        }
    }
    let mut ma = 0;
    for i in 0..2 {
        for j in 0..2 {
            ma = max(ma, dp[n][0][i][j] + max(i, j) as i32);
        }
    }
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
