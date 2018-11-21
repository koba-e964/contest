#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

const INF: i64 = 1 << 53;

// Fills dp[1][l..r].
fn compute(i: usize, l: usize, r: usize, lidx: usize, ridx: usize,
           dp: &mut [Vec<i64>], b: &[i64]) {
    if l >= r { return; }
    let mid = (l + r) / 2;
    let mut mi = (INF, ridx);
    for k in lidx .. min(ridx, mid) {
        let val = dp[0][k] + (mid - k) as i64 * (b[mid] - b[k]);
        mi = min(mi, (val, k));
    }
    dp[1][mid] = mi.0;
    compute(i, l, mid, lidx, mi.1 + 1, dp, b);
    compute(i, mid + 1, r, mi.1, ridx, dp, b);
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }
    let mut a = a;
    a.sort();
    let mut b = vec![0; n + 1];
    for i in 0 .. n { b[i + 1] = b[i] + a[i]; }
    let mut dp = vec![vec![0; n + 1]; 2];
    for i in 1 .. n + 1 { dp[0][i] = INF; }
    for i in 0 .. m {
        compute(i + 1, 1, n + 1, 0, n, &mut dp, &b);
        dp.swap(0, 1);
    }
    puts!("{}\n", dp[0][n]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
