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

/**
 * Returns the least index of elements that are modified, wrapped with Some.
 * If the entire array is reversed, it returns None instead.
 * v's elements must be pairwise distinct.
 */
fn next_permutation<T: Ord>(v: &mut [T]) -> Option<usize> {
    let mut tail_dec: usize = 1;
    let n = v.len();
    while tail_dec < n {
        if v[n - tail_dec - 1] > v[n - tail_dec] {
            tail_dec += 1;
        } else {
            break;
        }
    }
    // v[n - tail_dec .. n] is strictly decreasing
    if tail_dec < n {
        let x = n - tail_dec - 1;
        let mut y = n;
        {
            let pivot = &v[x];
            for i in (n - tail_dec .. n).rev() {
                if v[i] > *pivot {
                    y = i;
                    break;
                }
            }
            assert!(y < n);
        }
        v.swap(x, y);
    }
    v[n - tail_dec .. n].reverse();
    if tail_dec < n {
        Some(n - tail_dec - 1)
    } else {
        None
    }
}

fn calc(a: &[i64]) -> i64 {
    let n = a.len();
    let mut b = vec![];
    let m = n / 2;
    let mut tot = 0;
    for i in 0..m {
        b.push((a[i], i));
    }
    b.sort();
    b.reverse();
    for i in 0..m {
        let (val, idx) = b[i];
        tot += (n - i - 1 - idx) as i64 * val;
    }
    b.clear();
    for i in 0..n - m {
        b.push((a[i + m], i + m));
    }
    b.sort();
    b.reverse();
    for i in 0..n - m {
        let (val, idx) = b[i];
        tot += (idx - i) as i64 * val;
    }
    tot
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut b = vec![];
    for i in 0..n {
        b.push((a[i], i));
    }
    b.sort();
    b.reverse();
    const INF: i64 = 1 << 50;
    let mut dp = vec![vec![-INF; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (val, idx) = b[i];
        for j in 0..i + 1 {
            let lft = i - j;
            // left
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j] + val * (idx as i64 - lft as i64).abs());
            // right
            dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j] +
                                   val * ((n - j - 1) as i64 - idx as i64).abs());
        }
    }
    let mut ans = 0;
    for i in 0..n + 1 {
        ans = max(ans, dp[n][i]);
    }
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
