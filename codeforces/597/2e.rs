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
    input!(h: [[usize; 10]; 10]);
    let mut dp = vec![0.0; 100];
    let mut dp_st = vec![0.0; 100];
    let mut tbl = vec![vec![0; 10]; 10];
    let mut cells = vec![];
    let mut lad = vec![None; 100];
    for i in 0..10 {
        if i % 2 == 0 {
            for j in 0..10 {
                tbl[i][j] = cells.len();
                if h[i][j] > 0 {
                    let x = i - h[i][j];
                    lad[cells.len()] = Some(tbl[x][j]);
                }
                cells.push((i, j));
            }
        } else {
            for j in (0..10).rev() {
                tbl[i][j] = cells.len();
                if h[i][j] > 0 {
                    let x = i - h[i][j];
                    lad[cells.len()] = Some(tbl[x][j]);
                }
                cells.push((i, j));
            }
        }
    }
    for i in 1..100 {
        let loopback = if i >= 6 { 0.0 } else { (6 - i) as f64 / 6.0 };
        let mut sum = 0.0;
        let mut cnt = 0.0;
        for j in 1..min(i, 6) + 1 {
            sum += dp[i - j];
            cnt += 1.0;
        }
        dp_st[i] = sum / cnt + 1.0 / (1.0 - loopback);
        dp[i] = dp_st[i];
        if let Some(to) = lad[i] {
            if dp[i] > dp_st[to] {
                dp[i] = dp_st[to];
            }
        }
    }
    puts!("{}\n", dp[99]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
