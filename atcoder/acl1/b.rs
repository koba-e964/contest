#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y; y = r;
    }
    x
}

fn ext_gcd(x: i64, y: i64) -> (i64, i64, i64) {
    if y == 0 {
        return (x, 1, 0);
    }
    let r = x % y;
    let q = x / y;
    let (g, a, b) = ext_gcd(y, r);
    (g, b, a - b * q)
}

// Verified by: https://atcoder.jp/contests/ttpc2019/submissions/23338287
// Verified by: https://atcoder.jp/contests/acl1/submissions/23895636
fn garner((a, m): (i64, i64), (b, n): (i64, i64)) -> i64 {
    assert!(0 <= a);
    assert!(0 <= b);
    if a == b {
        return a;
    }
    let (g, mut x, mut y) = ext_gcd(m, n);
    assert_eq!(a % g, b % g);
    let m = m / g;
    let n = n / g;
    let q0 = a / g;
    let q1 = b / g;
    x %= n;
    if x < 0 {
        x += n;
    }
    y %= m;
    if y < 0 {
        y += m;
    }
    let rem0 = (q0 as i128 * y as i128) % m as i128;
    let rem1 = (q1 as i128 * x as i128) % n as i128;
    let val = rem0 as i64 * n + rem1 as i64 * m;
    let ret = val * g + (a % g);
    assert_eq!(ret % m, a % m);
    assert_eq!(ret % n, b % n);
    ret % (m / g * n)
}

// Tags: factorization, tricky-problem
fn main() {
    input!(n: i64);
    let mut d = 1;
    let mut divs = vec![];
    while d * d <= 2 * n {
        if 2 * n % d == 0 {
            if gcd(d, 2 * n / d) == 1 {
                divs.push((d, 2 * n / d));
                divs.push((2 * n / d, d));
            }
        }
        d += 1;
    }
    let mut ans = n;
    for (d1, d2) in divs {
        let x = garner((0, d1), (d2 - 1, d2));
        if x > 0 {
            ans = min(ans, x);
        }
    }
    println!("{}", ans);
}
