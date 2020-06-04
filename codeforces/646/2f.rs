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

// Tags: dp rotation substring subarray
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        nst: [(usize, chars, chars)],
    }
    const INF: i32 = 1 << 28;
    for (n, s, t) in nst {
        let mut occ_s = vec![[0; 26]; n + 1];
        let mut occ_t = vec![[0; 26]; n + 1];
        for i in 0..n {
            let idx = (s[i] as u8 - b'a') as usize;
            occ_s[i + 1] = occ_s[i];
            occ_s[i + 1][idx] += 1;
        }
        for i in 0..n {
            let idx = (t[i] as u8 - b'a') as usize;
            occ_t[i + 1] = occ_t[i];
            occ_t[i + 1][idx] += 1;
        }
        if occ_s[n] != occ_t[n] {
            puts!("-1\n");
            continue;
        }
        let mut dp = vec![vec![-INF; n + 1]; n + 1];
        dp[0][0] = 0;
        for i in 0..n + 1 {
            for j in 0..n + 1 {
                if i < n {
                    let c = (s[i] as u8 - b'a') as usize;
                    let ok =
                        occ_s[i + 1][c] <= occ_t[j][c];
                    if ok {
                        dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
                    }
                    if j < n {
                        if s[i] == t[j] {
                            dp[i + 1][j + 1] = max(dp[i + 1][j + 1],
                                                   dp[i][j] + 1);
                        }
                    }
                }
                if j < n {
                    dp[i][j + 1] = max(dp[i][j + 1], dp[i][j]);
                }
            }
        }
        let ma = dp[n][n];
        assert!(ma >= 0);
        puts!("{}\n", n - ma as usize);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
