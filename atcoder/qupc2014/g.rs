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

// Convex hull.
// Returns the points of the convex hull in the counter-clockwise order.
// Verified by: AGC021 B
// https://atcoder.jp/contests/agc021/submissions/22697344
type Coord = i64; // the type of coordinates
type P = (Coord, Coord);

fn det((ax, ay): P, (bx, by): P) -> Coord {
    ax * by - ay * bx
}

fn sub((ax, ay): P, (bx, by): P) -> P {
    (ax - bx, ay - by)
}

fn convex_hull(ps: &[P]) -> Vec<P> {
    let n = ps.len();
    if n == 0 {
        return vec![];
    }
    let mut k = 0;
    let mut ps = ps.to_vec();
    ps.sort();
    let mut ch = vec![(0, 0); 2 * n];
    // lower
    for i in 0..n {
        while k >= 2 &&
            det(sub(ps[i], ch[k - 2]), sub(ch[k - 1], ch[k - 2])) >= 0 {
            k -= 1;
        }
        ch[k] = ps[i];
        k += 1;
    }
    // upper
    let t = k + 1;
    for i in (0..n - 1).rev() {
        while k >= t &&
            det(sub(ps[i], ch[k - 2]), sub(ch[k - 1], ch[k - 2])) >= 0 {
            k -= 1;
        }
        ch[k] = ps[i];
        k += 1;
    }
    ch.truncate(k - 1);
    ch
}

fn collide_seg((cx, cy): (i64, i64), cr: i64,
               (a, b): (i64, i64), d: i64) -> bool {
    assert!(b <= d);
    let mid = max(b, min(cy, d));
    let sqr = |x| x * x;
    let dist = sqr(a - cx) + sqr(mid - cy);
    dist <= sqr(cr)
}

fn collide((cx, cy): (i64, i64), cr: i64,
           (a, b): (i64, i64), (c, d): (i64, i64)) -> bool {
    if a <= cx && cx <= c && b <= cy && cy <= d {
        return true;
    }
    collide_seg((cx, cy), cr, (a, b), d) ||
        collide_seg((cx, cy), cr, (c, b), d) ||
        collide_seg((cy, cx), cr, (b, a), c) ||
        collide_seg((cy, cx), cr, (d, a), c)
}

// Tags: geometry
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        c: [(i64, i64, i64); n],
        b: [(i64, i64, i64, i64); m],
    }
    let mut pts = vec![];
    for (x, y, w, h) in b {
        if c.iter().any(|&(cx, cy, cr)| collide((cx, cy), cr, (x, y), (x + w, y + h))) {
            for &a in &[x, x + w] {
                for &b in &[y, y + h] {
                    pts.push((a, b));
                }
            }
        }
    }
    let ch = convex_hull(&pts);
    let m = ch.len();
    let mut ans = 0.0;
    for i in 0..m {
        let sqr = |x| x * x;
        let dist = sqr(ch[(i + 1) % m].0 - ch[i].0) + sqr(ch[(i + 1) % m].1 - ch[i].1);
        ans += (dist as f64).sqrt();
    }
    puts!("{}\n", ans);
}
