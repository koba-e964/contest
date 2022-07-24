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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn cand(x: i64, y: i64, b: i64) -> Vec<(i64, i64)> {
    let qx = x.div_euclid(b) * b;
    let qy = y.div_euclid(b) * b;
    vec![
        (qx, y),
        (qx + b, y),
        (x, qy),
        (x, qy + b),
    ]
}

pub fn dist(ax: i64, ay: i64, bx: i64, by: i64, b: i64) -> i64 {
    if ax.div_euclid(b) != bx.div_euclid(b) &&
        ay.div_euclid(b) != by.div_euclid(b) {
            return (ax - bx).abs() + (ay - by).abs();
        }
    if ax == bx && ay.div_euclid(b) == by.div_euclid(b) {
        return (ax - bx).abs() + (ay - by).abs();
    }
    if ay == by && ax.div_euclid(b) == bx.div_euclid(b) {
        return (ax - bx).abs() + (ay - by).abs();
    }
    if ax.div_euclid(b) == bx.div_euclid(b) {
        let tmp1 = ax.rem_euclid(b);
        let tmp2 = bx.rem_euclid(b);
        return (ay - by).abs() + min(tmp1 + tmp2, 2 * b - tmp1 - tmp2);
    }
    let tmp1 = ay.rem_euclid(b);
    let tmp2 = by.rem_euclid(b);
    (ax - bx).abs() + min(tmp1 + tmp2, 2 * b - tmp1 - tmp2)
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        cases: [(i64, i64, i64, i64, i64, i64); t],
    }
    for (b, k, sx, sy, gx, gy) in cases {
        let mut mi = (sx - gx).abs() + (sy - gy).abs();
        mi *= k;
        let gcand = cand(gx, gy, b);
        for (ax, ay) in cand(sx, sy, b) {
            for &(bx, by) in &gcand {
                let mut tmp = (sx - ax).abs() + (sy - ay).abs()
                    + (gx - bx).abs() + (gy - by).abs();
                tmp *= k;
                tmp += dist(ax, ay, bx, by, b);
                mi = min(mi, tmp);
            }
        }
        puts!("{}\n", mi);
    }
}
