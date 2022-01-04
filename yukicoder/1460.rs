use std::cmp::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const INF: i64 = 1 << 60;

fn mul(a1: &[i64], a2: &[i64], b: &[i64]) -> Vec<i64> {
    let k = a1.len();
    let mut out = vec![-INF; 2 * k];
    for i in 0..k {
        for j in 0..k {
            out[i + j] = max(out[i + j], min(a1[i], a2[j]));
        }
    }
    for i in (k..2 * k).rev() {
        for j in 0..k {
            out[i - k + j] = max(out[i - k + j], min(b[j], out[i]));
        }
    }
    out.truncate(k);
    out
}

fn pow(a: &[i64], mut e: i64, b: &[i64]) -> Vec<i64> {
    let k = a.len();
    let mut prod = vec![-INF; k];
    prod[0] = INF;
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            prod = mul(&prod, &cur, b);
        }
        cur = mul(&cur, &cur, b);
        e /= 2;
    }
    prod
}

// https://yukicoder.me/problems/no/1460 (3.5)
// (max, min)-半環上の行列累乗で O(K^3 log N)。Kitamasa 法が使えれば O(K^2 log N)。
// Tags: tropical-semiring, matrix-exponentiation-on-semiring, kitamasa
fn main() {
    input! {
        k: usize, n: i64,
        a: [i64; k],
        b: [i64; k],
    }
    let mut x = vec![-INF; k];
    if k > 1 {
        x[1] = INF;
    } else {
        x[0] = b[0];
    }
    let s = pow(&x, n, &b);
    let mut ans = -INF;
    for i in 0..k {
        ans = max(ans, min(s[i], a[i]));
    }
    println!("{}", ans);
}
