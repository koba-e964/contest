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

// Tags: constant-factor optimization, 500^3
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, k: usize,
        ab: [(i64, i64); n],
    }
    const INF: i64 = 1 << 50;
    let mut dp = vec![vec![-INF; k]; 2];
    dp[0][0] = 0;
    let mut cur = 0;
    for i in 0..n {
        let t = i % 2;
        for j in 0..k {
            dp[1 - t][j] = -INF;
        }
        let (a, b) = ab[i];
        for j in 0..k {
            let l = (k + cur - j) % k;
            if dp[t][j] <= -INF { continue; }
            let mut dy = (a + b) as usize % k;
            for dx in 0..min(a as usize + 1, k) {
                if dy > b as usize {
                    dy += k - 1;
                    if dy >= k { dy -= k; }
                    continue;
                }
                let now = ((a + b) as usize - dx - dy) / k;
                let nx = (j + dx) % k;
                let added = (j + dx) / k + (l + dy) / k;
                dp[1 - t][nx] = max(dp[1 - t][nx],
                                    dp[t][j] + now as i64 + added as i64);
                dy += k - 1;
                if dy >= k { dy -= k; }
            }
            
        }
        cur = ((cur as i64 + a + b) % k as i64) as usize;
    }
    let mut ma = 0;
    for i in 0..k {
        ma = max(ma, dp[n % 2][i]);
    }
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
