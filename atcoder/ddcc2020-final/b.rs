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

const INF: i64 = 1 << 58;

type Fun = (i64, i64); // (a, b): x |-> max(x + a, b)

fn comp((a, b): Fun, (c, d): Fun) -> Fun {
    // max(max(x + a, b) + c, d)
    (a + c, max(b + c, d))
}

fn fmax((a, b): Fun, (c, d): Fun) -> Fun {
    // max(max(x + a, b), max(x + c, d))
    (max(a, c), max(b, d))
}

fn mul(a: &[Vec<Fun>], b: &[Vec<Fun>]) -> Vec<Vec<Fun>> {
    let n = a.len();
    let mut ret = vec![vec![(-INF, -INF); n]; n];
    for i in 0..n {
        for j in 0..n {
            if a[i][j].0 <= -INF { continue; }
            for k in 0..n {
                if b[j][k].0 > -INF {
                    ret[i][k] = fmax(ret[i][k], comp(a[i][j], b[j][k]));
                }
            }
        }
    }
    ret
}

fn pow(a: &[Vec<Fun>], mut e: i64) -> Vec<Vec<Fun>> {
    let n = a.len();
    let mut prod = vec![vec![(-INF, -INF); n]; n];
    let mut cur = a.to_vec();
    for i in 0..n {
        prod[i][i] = (0, -INF);
    }
    while e > 0 {
        if e % 2 == 1 {
            prod = mul(&prod, &cur);
        }
        cur = mul(&cur, &cur);
        e /= 2;
    }
    prod
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize, w: i64, s: usize1, k: i64,
        abc: [(usize1, usize1, i64); m],
    }
    let mut mat = vec![vec![(-INF, -INF); n]; n];
    for &(a, b, c) in &abc {
        mat[a][b] = (c, 0);
    }
    let ans = pow(&mat, k);
    let mut ma = -INF;
    for i in 0..n {
        ma = max(ma, max(ans[s][i].0 + w, ans[s][i].1));
    }
    if ma <= -INF / 2 {
        puts!("{}\n", -1);
    } else {
        puts!("{}\n", ma);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
