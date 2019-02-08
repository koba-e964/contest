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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        m: usize,
        k: usize,
        stdw: [(usize, usize, usize, i64); k],
    }
    let mut nxt = vec![(0, 0); n];
    {
        let mut events = vec![vec![]; n + 1];
        for (i, &(s, t, d, w)) in stdw.iter().enumerate() {
            events[s - 1].push((i, 0, d, w));
            events[t].push((i, 1, d, w));
        }
        let mut dat = BTreeSet::new();
        for i in 0..n {
            for &(idx, kind, d, w) in &events[i] {
                match kind {
                    0 => {
                        dat.insert((w, d, idx));
                    }
                    1 => {
                        dat.remove(&(w, d, idx));
                    }
                    _ => unreachable!(),
                }
            }
            nxt[i] = match dat.range(..).rev().next() {
                None => (i + 1, 0),
                Some(&(w, d, _idx)) => (d, w),
            };
        }
    }
    const INF: i64 = 1 << 50;
    let mut dp = vec![vec![INF; m + 1]; n + 1];
    for j in 0..m + 1 { dp[n][j] = 0; }
    for i in (0..n).rev() {
        for j in 0..m + 1 {
            let (d, w) = nxt[i];
            dp[i][j] = min(dp[i][j], dp[d][j] + w);
            if j < m {
                dp[i][j] = min(dp[i][j], dp[i + 1][j + 1]);
            }
        }
    }
    puts!("{}\n", dp[0][0]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
