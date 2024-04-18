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
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Online monotone minima dp. For example, monge dp can be efficiently computed
// by online_dc.
// Verified by: https://yukicoder.me/problems/no/705
// submission: https://yukicoder.me/submissions/566775
const INF: i64 = 1 << 60;

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

// b must be convex.
fn min_plus_conv(a: &[i64], b: &[i64]) -> Vec<i64> {
    let n = a.len();
    let m = b.len();
    for i in 2..m {
        assert!(b[i - 2] - b[i - 1] >= b[i - 1] - b[i]);
    }
    let mut dp = vec![INF; n + m - 1];
    let cost_fun = |j: usize, i: usize| {
        if i >= j && i < j + m {
            b[i - j]
        } else {
            INF / 2
        }
    };
    monotone_minima(0, n, 0, n + m - 1, &a, &mut dp, &cost_fun);
    dp
}

// (tbl, pure)
fn rec(ab: &[(i64, i64)]) -> (Vec<i64>, Vec<i64>) {
    if ab.len() == 1 {
        let (a, b) = ab[0];
        return (vec![-a + b], vec![0, -a]);
    }
    let mid = ab.len() / 2;
    let (l, r) = ab.split_at(mid);
    let (tbl_l, pure_l) = rec(l);
    let (tbl_r, pure_r) = rec(r);
    let pure = min_plus_conv(&pure_l, &pure_r);
    let mut tbl = min_plus_conv(&tbl_r, &pure_l);
    for i in 0..tbl_l.len() {
        tbl[i] = std::cmp::min(tbl[i], tbl_l[i]);
    }
    (tbl, pure)
}

// Solved with hints
// Tags: min-plus-convolution, divide-and-conquer, monotone-minima
fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut ab = ab;
    ab.sort_by_key(|&(_, b)| b);
    let (tbl, _pure) = rec(&ab);
    for i in 0..n {
        println!("{}", -tbl[i]);
    }
}
