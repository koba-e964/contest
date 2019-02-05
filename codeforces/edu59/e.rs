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

const INF: i64 = 1 << 50;

fn chmax(x: &mut i64, y: i64) {
    *x = max(*x, y);
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        s: chars,
        a: [i64; n],
    }
    let s: Vec<usize> = s.into_iter().map(|x| x as usize - 48).collect();
    let mut dp = vec![vec![-INF; n + 1]; n];
    // aux[i][j][u][k]: Considering [i, j), picked k letters of u.
    let mut aux = vec![vec![vec![vec![-INF; n + 1]; 2]; n + 1]; n];
    for i in (0..n).rev() {
        dp[i][i] = 0;
        for j in i + 1..n + 1 {
            let u = s[j - 1];
            // Pick u
            chmax(&mut aux[i][j][u][1], dp[i][j - 1]);
            for k in 0..n {
                let val = aux[i][j - 1][u][k];
                chmax(&mut aux[i][j][u][k + 1], val);
            }
            // pick dp[l][j]
            for k in 0..n + 1 {
                for l in i + 1..j {
                    for u in 0..2 {
                        let val = aux[i][l][u][k] + dp[l][j];
                        chmax(&mut aux[i][j][u][k], val);
                    }
                }
            }
            // resolve
            for k in 1..n + 1 {
                for u in 0..2 {
                    chmax(&mut dp[i][j], aux[i][j][u][k] + a[k - 1]);
                }
            }
        }
    }
    puts!("{}\n", dp[0][n]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
