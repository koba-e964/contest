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
            bytes.by_ref().map(|r|r.unwrap() as char)
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
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

const INF: i64 = 1 << 60;

fn monotone_minima(frm: &[i64], lat: &mut [i64],
                   a: &[i64], x: &[i64], y: &[i64]) {
    let n = frm.len();
    let m = lat.len();
    if n == 0 || m == 0 {
        return;
    }
    assert_eq!(a.len(), m);
    assert_eq!(x.len(), n);
    assert_eq!(y.len(), n);
    assert_ne!(n, 0);
    let half = m / 2;
    let mut mi = (INF, n);
    for i in 0..n {
        let cost = (a[half] - x[i]) * (a[half] - x[i]) + y[i] * y[i];
        mi = min(mi, (cost + frm[i], i));
    }
    let idx = mi.1;
    assert!(idx < n);
    lat[half] = min(lat[half], mi.0);
    monotone_minima(&frm[..idx + 1], &mut lat[..half],
                    &a[..half], &x[..idx + 1], &y[..idx + 1]);
    monotone_minima(&frm[idx..], &mut lat[half + 1..],
                    &a[half + 1..], &x[idx..], &y[idx..]);
}

fn induce(l: usize, mid: usize, r: usize, dp: &mut [i64],
          a: &[i64], x: &[i64], y: &[i64]) {
    let (frm, lat) = dp.split_at_mut(mid);
    let frm = &frm[l..];
    let lat = &mut lat[..r - mid];
    monotone_minima(frm, lat, &a[mid..r], &x[l..mid], &y[l..mid]);
}

// Performs online dp.
// Converted from the following off-line dp:
// dp[i + 1][j] <--min-- dp[i][k] + (a[j] - x[k + 1])^2  (k < j)
fn dfs(l: usize, r: usize, dp: &mut [i64],
       a: &[i64], x: &[i64], y: &[i64]) {
    if l + 1 >= r {
        return;
    }
    let mid = (l + r) / 2;
    dfs(l, mid, dp, a, x, y);
    // induce
    induce(l, mid, r, dp, a, x, y);
    dfs(mid, r, dp, a, x, y);
}

// Tags: online-dp
// https://qiita.com/tmaehara/items/0687af2cfb807cde7860
// https://ei1333.github.io/luzhiled/snippets/dp/monotone-minima.html
// O(n log^2 n)
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        a: [i64; n],
        x: [i64; n],
        y: [i64; n],
    }
    let mut a = a;
    a.insert(0, 0);
    let mut x = x;
    x.push(0);
    let mut y = y;
    y.push(0);
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    dfs(0, n + 1, &mut dp, &a, &x, &y);
    puts!("{}\n", dp[n]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
