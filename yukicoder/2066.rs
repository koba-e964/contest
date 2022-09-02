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

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

// Ported from ACL.
// Computes \sum_{i = 0}^{n - 1} floor((a * i + b) / m).
// Verified by: https://atcoder.jp/contests/arc111/submissions/23877969
fn floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64 {
    fn internal(n: i64, m: i64, mut a: i64, mut b: i64, mut acc: i64) -> i64 {
        if a >= m {
            let q = a / m;
            acc += (n - 1) * n / 2 * q;
            a -= q * m;
        }
        if b >= m {
            let q = b / m;
            acc += n * q;
            b -= q * m;
        }
        let y_max = (a * n + b) / m;
        let x_max = y_max * m - b;
        if y_max == 0 {
            return acc;
        }
        acc += (n - (x_max + a - 1) / a) * y_max;
        let mut sub_b = a - x_max % a;
        if sub_b >= a {
            sub_b -= a;
        }
        internal(y_max, a, m, sub_b, acc)
    }
    internal(n, m, a, b, 0)
}

// gcd(a, b) = 1
fn calc(a: i64, b: i64, k: i64) -> i64 {
    if a == 1 || b == 1 {
        return k;
    }
    let lim = a * b - a - b; // lim >= 0
    let cnt = floor_sum(lim / b + 1, a, b, lim - (lim / b) * b) + lim / b;
    if cnt < k {
        return lim + k - cnt;
    }
    let mut fail = -1;
    let mut pass = lim;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let tmp = floor_sum(mid / b + 1, a, b, mid - (mid / b) * b) + mid / b;
        if tmp >= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    pass
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        pqk: [(i64, i64, i64); t],
    }
    for (p, q, k) in pqk {
        let g = gcd(p, q);
        puts!("{}\n", calc(p / g, q / g, k) * g);
    }
}
