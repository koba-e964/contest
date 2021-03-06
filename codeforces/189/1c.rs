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

/*
 * Online monotone minima dp. For example, monge dp can be efficiently computed
 * by online_dc.
 * Verified by: https://yukicoder.me/problems/no/705
 * submission: https://yukicoder.me/submissions/566775
 */
// We want INF to be greater than any possible value of cost + dp[i].
const INF: i64 = 1 << 61;

// Complexity: O(n log m + m) where n = r - l, m = b - a.
fn monotone_minima<F>(l: usize, r: usize, a: usize, b: usize,
                      frm: &[i64], lat: &mut [i64],
                      cost_fun: &F)
where F: Fn(usize, usize) -> i64 {
    let n = r - l;
    let m = b - a;
    if n == 0 || m == 0 {
        return;
    }
    let mid = (a + b) / 2;
    let mut mi = (INF, n);
    for i in l..r {
        let cost = cost_fun(i, mid);
        mi = std::cmp::min(mi, (cost + frm[i], i));
    }
    let idx = mi.1;
    assert!(l <= idx && idx < r);
    lat[mid] = std::cmp::min(lat[mid], mi.0);
    monotone_minima(l, idx + 1, a, mid, frm, lat, cost_fun);
    monotone_minima(idx, r, mid + 1, b, frm, lat, cost_fun);
}

// Takes O(n log n)-time where n = r - l.
fn induce<F>(l: usize, mid: usize, r: usize, dp: &mut [i64],
             cost_fun: &F)
where F: Fn(usize, usize) -> i64 {
    let (frm, lat) = dp.split_at_mut(mid);
    let frm = &frm[l..];
    let lat = &mut lat[..r - mid];
    let inner_cost_fun = |i: usize, j: usize| cost_fun(i + l, j + mid);
    monotone_minima(0, mid - l, 0, r - mid,
                    frm, lat, &inner_cost_fun);
}

// Performs online dp with divide and conquer.
// Converted from the following off-line dp:
// dp[i + 1][j] <--min-- dp[i][k] + cost_fun(k, j)  (k < j)
// Complexity: O(n log^2 n) where n = r - l
fn online_dc<F>(l: usize, r: usize, dp: &mut [i64],
                cost_fun: &F)
where F: Fn(usize, usize) -> i64 {
    if l + 1 >= r {
        return;
    }
    let mid = (l + r) / 2;
    online_dc(l, mid, dp, cost_fun);
    induce(l, mid, r, dp, cost_fun);
    online_dc(mid, r, dp, cost_fun);
}

// Tags: online-divide-and-conquer, monge-dp, monotone-minima, n-log-2-n, 10^5
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
        b: [i64; n],
    }
    // Satisfies Monge property.
    let cost_fun = |i: usize, k: usize| {
        b[i] * a[k]
    };
    let mut dp = vec![INF; n];
    dp[0] = 0;
    online_dc(0, n, &mut dp, &cost_fun);
    puts!("{}\n", dp[n - 1]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
