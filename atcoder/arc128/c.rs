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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

// Tags: linear-programming
// Ref: https://www.slideshare.net/wata_orz/ss-91375739
fn calc(b: &[i64], m: i64, s: i64, mid: f64) -> (f64, usize) {
    let n = b.len();
    let mut tmp = (-1.0e20, 0);
    for i in 0..n {
        tmp.chmax((b[i] as f64 + mid * (n - i) as f64, i));
    }
    let idx = tmp.1;
    if tmp.0 <= 0.0 {
        (-mid * s as f64, n)
    } else {
        (tmp.0 * m as f64 - mid * s as f64, idx)
    }
}

fn main() {
    input! {
        n: usize, m: i64, s: i64,
        a: [i64; n],
    }
    let mut b = vec![0; n];
    b[n - 1] = a[n - 1];
    for i in (0..n - 1).rev() {
        b[i] = b[i + 1] + a[i];
    }
    let mut lo = -1.0e14;
    let mut hi = 1.0e14;
    for _ in 0..100 {
        let mid = (lo + hi) / 2.0;
        let (_, idx) = calc(&b, m, s, mid);
        if ((n - idx) as i64) * m >= s {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    println!("{}", calc(&b, m, s, lo).0);
}
