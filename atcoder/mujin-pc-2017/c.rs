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

// This solution was written after the author read the editorial.
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        s: chars,
        lr: [(usize1, usize)],
    }
    let n = s.len();
    let s: Vec<_> = s.into_iter().map(|x| (x as u8 - b'a') as usize).collect();
    let mut dp = vec![n + 1; n + 1];
    let mut dps = vec![vec![n + 1; 26]; n + 1];
    for i in (0..n).rev() {
        let mut que = BinaryHeap::new();
        que.push((n - i - 1, s[i]));
        while let Some((pidx, ch)) = que.pop() {
            let idx = n - pidx;
            if ch <= 25 {
                if dps[i][ch] <= idx { continue; }
                dps[i][ch] = idx;
                if dps[idx][ch] <= n {
                    que.push((n - dps[idx][ch], ch + 1));
                }
            } else {
                assert_eq!(ch, 26);
                if dp[i] <= idx { continue; }
                dp[i] = idx;
                for j in 0..26 {
                    if dps[idx][j] <= n {
                        que.push((n - dps[idx][j], j));
                    }
                }
            }
        }
    }
    const B: usize = 20;
    let mut dbl = vec![vec![n + 1; n]; B];
    for i in 0..n {
        dbl[0][i] = dp[i];
    }
    for j in 1..B {
        for i in 0..n {
            let stage = dbl[j - 1][i];
            if stage < n {
                dbl[j][i] = dbl[j - 1][stage];
            }
        }
    }
    for (mut l, r) in lr {
        for i in (0..B).rev() {
            if dbl[i][l] <= r {
                l = dbl[i][l];
            }
            if l == n { break; }
        }
        puts!("{}\n", if l == r { "Yes" } else { "No" });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
