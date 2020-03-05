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

fn calc(a: &[i64], x: i64) -> u64 {
    if x < 0 {
        return 0;
    }
    let n = a.len();
    let m = 61;
    let mut rest = vec![3; m];
    for i in 0..n - 1 {
        let d = a[i] ^ a[i + 1];
        let mut hi = m;
        for i in (0..m).rev() {
            if (d & 1 << i) != 0 {
                hi = i;
                break;
            }
        }
        rest[hi] &= if (a[i] & 1 << hi) != 0 { 2 } else { 1 };
    }
    let mut dp = vec![[0u64; 2]; m + 1];
    dp[m][1] = 1;
    for i in (0..m).rev() {
        for j in 0..2 {
            if (rest[i] & 1 << j) == 0 { continue; }
            for k in 0..2 {
                if k == 1 && j > (x >> i & 1) as usize {
                    continue;
                }
                let nk = k & if j == (x >> i & 1) as usize { 1 } else { 0 };
                dp[i][nk] += dp[i + 1][k];
            }
        }
    }
    dp[0][0] + dp[0][1]
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, l: i64, r: i64,
        a: [i64; n],
    }
    puts!("{}\n", calc(&a, r) - calc(&a, l - 1));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
