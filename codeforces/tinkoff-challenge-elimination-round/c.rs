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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

const INF: i64 = 1 << 30;

// Verified by https://atcoder.jp/contests/cpsco2019-s3/submissions/5279150
#[derive(Clone, Copy, PartialEq, Eq)]
struct Frac {
    x: i64, y: i64,
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y; y = r;
    }
    x.abs()
}

#[allow(unused)]
impl Frac {
    fn new() -> Self {
        Frac {x: 0, y: 1}
    }
    fn add(self, o: Self) -> Self {
        Frac { x: self.x * o.y + self.y * o.x, y: o.y * self.y }.red()
    }
    fn sub(self, o: Self) -> Self {
        Frac { x: self.x * o.y - self.y * o.x, y: o.y * self.y }.red()
    }
    fn mul(self, o: Self) -> Self {
        Frac { x: self.x * o.x, y: self.y * o.y }.red()
    }
    fn div(self, o: Self) -> Self {
        Frac { x: self.x * o.y, y: self.y * o.x }.red()
    }
    fn neg(self) -> Self {
        Frac {x: -self.x, y: self.y }
    }
    fn red(mut self) -> Self {
        let g = gcd(self.x, self.y);
        self.x /= g;
        self.y /= g;
        if self.y < 0 {
            Frac {x: -self.x, y: -self.y }
        } else {
            self
        }
    }
}

fn frac(x: i64, y: i64) -> Frac {
    Frac {x: x, y: y}.red()
}

impl Ord for Frac {
    fn cmp(&self, o: &Frac) -> std::cmp::Ordering {
        (self.x * o.y).cmp(&(self.y * o.x))
    }
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, o: &Frac) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}
impl std::fmt::Debug for Frac {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.x, self.y)
    }
}
impl std::fmt::Display for Frac {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.x, self.y)
    }
}

fn calc(mut r: Frac, mut v: Frac, mut lo: Frac, mut hi: Frac) -> (Frac, Frac) {
    if v == frac(0, 1) {
        return if lo < r && r < hi {
            (frac(0, 1), frac(INF, 1))
        } else {
            (frac(1, 1), frac(0, 1))
        };
    }
    if v < frac(0, 1) {
        r = r.neg();
        v = v.neg();
        let (lot, hit) = (hi.neg(), lo.neg());
        lo = lot;
        hi = hit;
    }
    let a = lo.sub(r).div(v);
    let b = hi.sub(r).div(v);
    (a, b)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        x1: i64, y1: i64, x2: i64, y2: i64,
        rv: [(i64, i64, i64, i64); n],
    }
    let mut lo = frac(0, 1);
    let mut hi = frac(INF, 1);
    for (rx, ry, vx, vy) in rv {
        let (a, b) = calc(frac(rx, 1), frac(vx, 1), frac(x1, 1), frac(x2, 1));
        lo = lo.max(a);
        hi = hi.min(b);
        let (a, b) = calc(frac(ry, 1), frac(vy, 1), frac(y1, 1), frac(y2, 1));
        lo = lo.max(a);
        hi = hi.min(b);
    }
    if lo >= hi {
        puts!("-1\n");
    } else {
        puts!("{:.15}\n", lo.x as f64 / lo.y as f64);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
