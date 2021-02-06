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
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
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

trait Change {
    fn chmax(&mut self, x: Self);
    fn chmin(&mut self, x: Self);
}
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) {
        if *self < x { *self = x; }
    }
    fn chmin(&mut self, x: T) {
        if *self > x { *self = x; }
    }
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn sqrt(x: i64) -> i64 {
    if x <= 1 {
        return x;
    }
    let mut hi = 1 << 31;
    let mut lo = 0;
    while hi - lo > 1 {
        let mid = (hi + lo) / 2;
        if mid * mid <= x {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

fn quo(x: i64, y: i64) -> i64 {
    assert!(y > 0);
    let q = x / y;
    let r = x - q * y;
    if r < 0 {
        q - 1
    } else {
        q
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        x: f64, y: f64, r: f64,
    }
    let x = (x * 10000.0 + 0.5).floor() as i64;
    let y = (y * 10000.0 + 0.5).floor() as i64;
    let r = (r * 10000.0 + 0.5).floor() as i64;
    let mut ans = 0i64;
    for i in -210000..210000 {
        let t = x - i * 10000;
        if r < t.abs() {
            continue;
        }
        let dist = (r - t) * (r + t);
        let dist = sqrt(dist);
        let hi = quo(y + dist, 10000);
        let lo = quo(y - dist + 9999, 10000);
        ans += hi - lo + 1;
    }
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
