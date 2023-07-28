use std::cmp::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Find the area of x^2 + y^2 <= a^2, (x-l)^2 + y^2 <= b^2
fn f(a: i64, b: i64, l: i64) -> f64 {
    if a + b <= l {
        return 0.0;
    }
    let pi = std::f64::consts::PI;
    if a + l <= b {
        return pi * (a * a) as f64;
    }
    if b + l <= a {
        return pi * (b * b) as f64;
    }
    let x0 = (l * l + a * a - b * b) as f64 / (2 * l) as f64;
    let height = (a as f64 * a as f64 - x0 * x0).sqrt();
    let mut ans = l as f64 * height;
    ans = -ans;
    let adeg = height.atan2(x0);
    ans += adeg * a as f64 * a as f64;
    let bdeg = height.atan2(l as f64 - x0);
    ans += bdeg * b as f64 * b as f64;
    ans
}

// https://yukicoder.me/problems/no/764 (3.5)
// 幾何。円から円をくりぬいたもの 2 個の共通部分なので、包除原理でできる。
// -> WA。扇形の面積を deg * r^2/2 ではなく pi * deg * r^2/2 と誤認していた。
// Tags: geometry, intersection-of-two-circles
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        suml: i64,
        l: [i64; n + 1],
    }
    let mut lft = vec![(0, 0); n];
    let mut ma = 0;
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += l[i - 1];
        ma = max(ma, l[i - 1]);
        lft[i - 1] = (max(0, 2 * ma - sum), sum);
    }
    let mut rgt = vec![(0, 0); n];
    let mut ma = 0;
    let mut sum = 0;
    for i in (1..n + 1).rev() {
        sum += l[i];
        ma = max(ma, l[i]);
        rgt[i - 1] = (max(0, 2 * ma - sum), sum);
    }
    for i in 0..n {
        let (a, b) = lft[i];
        let (c, d) = rgt[i];
        puts!("{}\n", f(b, d, suml) - f(a, d, suml) - f(b, c, suml) + f(a, c, suml));
    }
}
