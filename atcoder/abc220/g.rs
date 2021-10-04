use std::cmp::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn perpendicular_bisector((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> (i64, i64, i64) {
    let x = x1 + x2;
    let y = y1 + y2;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut a = 2 * dx;
    let mut b = 2 * dy;
    let mut c = dx * x + dy * y;
    let g = gcd(gcd(a, b), c);
    a /= g;
    b /= g;
    c /= g;
    if (a, b) < (0, 0) {
        a = -a;
        b = -b;
        c = -c;
    }
    // ax + by = c, (a, b) > (0, 0)
    (a, b, c)
}

fn line((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> (i64, i64, i64) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut a = -dy;
    let mut b = dx;
    let mut c = -dy * x1 + dx * y1;
    let g = gcd(gcd(a, b), c);
    a /= g;
    b /= g;
    c /= g;
    if (a, b) < (0, 0) {
        a = -a;
        b = -b;
        c = -c;
    }
    // ax + by = c, (a, b) > (0, 0)
    (a, b, c)
}

fn calc(v: &[((i64, i64, i64), i64)]) -> i64 {
    let mut hm = HashMap::new();
    for &(line, sc) in v {
        hm.entry(line).or_insert(0).chmax(sc);
    }
    let mut tmp = vec![];
    for (_line, sc) in hm {
        tmp.push(sc);
    }
    tmp.sort(); tmp.reverse();
    if tmp.len() >= 2 {
        tmp[0] + tmp[1]
    } else {
        -1
    }
}

// Tags: perpendicular-bisector
fn main() {
    input! {
        n: usize,
        xyc: [(i64, i64, i64); n],
    }
    let mut hm = HashMap::new();
    for i in 0..n {
        let (xi, yi, ci) = xyc[i];
        for j in 0..i {
            let (xj, yj, cj) = xyc[j];
            let perp = perpendicular_bisector((xi, yi), (xj, yj));
            let line = line((xi, yi), (xj, yj));
            hm.entry(perp).or_insert(vec![]).push((line, ci + cj));
        }
    }
    let mut ma = -1;
    for (_, v) in hm {
        ma = max(ma, calc(&v));
    }
    println!("{}", ma);
}
