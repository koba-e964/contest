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

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y; y = r;
    }
    x
}

fn extgcd(x: i64, y: i64) -> (i64, i64, i64) {
    if y == 0 {
        return (x, 1, 0)
    }
    let q = x / y;
    let r = x - q * y;
    let (g, a, b) = extgcd(y, r);
    (g, b, a - q * b)
}

fn quo(x: i64, y: i64) -> i64 {
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
        xypq: [(i64, i64, i64, i64)],
    }
    for &(x, y, p, q) in &xypq {
        let g = gcd(2 * (x + y), p + q);
        let lo = p - x - y;
        let hi = p + q - x;
        let loq = quo(lo + g, g);
        let hiq = quo(hi + g - 1, g);
        if loq >= hiq {
            puts!("infinity\n");
            continue;
        }
        let c = 2 * (x + y) / g;
        let d = (p + q) / g;
        let (_g, e, _f) = extgcd(c, d);
        assert_eq!(_g, 1);
        let mut mi = 1 << 62;
        for i in loq..hiq {
            // Find a, b s.t. c * b - d * a = i
            let mut b = e * i;
            b %= d;
            if b < 0 {
                if b < 0 {
                    b += d;
                }
            }
            assert_eq!((c * b - i) % d, 0);
            let mut a = (c * b - i) / d;
            if a < 0 {
                let q = -quo(a, c);
                a += q * c;
                b += q * d;
            }
            if false {
                eprintln!("c = {}, d = {}, a = {}, b = {}, targ = {}",
                      c, d, a, b, i);
                eprintln!("train: {} {}, tak: {} {}",2 * (x + y) * b + x, 2 * (x + y) * b + x + y, (p + q) * a + p, (p + q) * a + p + q);
            }
            mi.chmin(max((p + q) * a + p, 2 * (x + y) * b + x));
        }
        puts!("{}\n", mi);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
