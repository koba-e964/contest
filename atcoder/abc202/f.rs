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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

type Coord = i64; // the type of coordinates
type P = (Coord, Coord);

fn det((ax, ay): P, (bx, by): P) -> Coord {
    ax * by - ay * bx
}

fn sub((ax, ay): P, (bx, by): P) -> P {
    (ax - bx, ay - by)
}

// The author read the editorial before implementing this.
// Editorial: https://atcoder.jp/contests/abc202/editorial/1868
// Tags: convex-hull, grahams-scan
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut xy = xy;
    xy.sort();
    let mut ans = 0u128;
    for left in 0..n {
        let mut pts: Vec<_> =
            xy.iter().skip(left).map(|&p| sub(p, xy[left])).collect();
        let m = pts.len();
        pts.sort_by(|&a, &b|
                    0.cmp(&det(a, b)));
        let mut count = vec![vec![0u128; m]; m];
        for i in 0..m {
            for j in i + 1..m {
                for k in i + 1..j {
                    if det(sub(pts[i], pts[k]), sub(pts[j], pts[k])) > 0 {
                        count[i][j] += 1;
                    }
                }
            }
        }
        let mut dp = vec![vec![[0u128; 2]; m]; m];
        for j in 1..m {
            dp[0][j][0] = 1;
        }
        for i in 0..m {
            for j in i + 1..m {
                for k in 0..2 {
                    for l in j + 1..m {
                        if det(sub(pts[j], pts[i]), sub(pts[l], pts[j])) > 0 {
                            dp[j][l][k ^ det(pts[j], pts[l]).rem_euclid(2) as usize] += dp[i][j][k] << count[j][l];
                        }
                    }
                }
            }
        }
        for i in 1..m {
            for j in i + 1..m {
                ans += dp[i][j][0];
            }
        }
    }
    puts!("{}\n", ans % 1_000_000_007);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
