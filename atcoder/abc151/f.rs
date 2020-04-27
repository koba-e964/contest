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

const EPS: f64 = 1.0e-9;

fn check(xy: &[(i64, i64)], x: f64, y: f64, r: f64) -> bool {
    for &(a, b) in xy {
        let a = a as f64 - x;
        let b = b as f64 - y;
        let dist = a * a + b * b;
        if dist > r * r + 2.0 * EPS * r {
            return false;
        }
    }
    true
}

fn dist(a: (i64, i64), b: (i64, i64)) -> f64 {
    let x = (a.0 - b.0) as f64;
    let y = (a.1 - b.1) as f64;
    (x * x + y * y).sqrt()
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut mi = 1.0e18f64;
    for i in 0..n {
        let (x1, y1) = xy[i];
        for j in 0..i {
            let (x2, y2) = xy[j];
            let x = (x1 + x2) as f64 / 2.0;
            let y = (y1 + y2) as f64 / 2.0;
            let r = dist(xy[i], xy[j]) / 2.0;
            if check(&xy, x, y, r) {
                mi = mi.min(r);
            }
            for k in 0..j {
                // http://blog.livedoor.jp/dgd17/archives/11821338.html
                // circumcenter of a triangle
                let eij = dist(xy[i], xy[j]);
                let eik = dist(xy[i], xy[k]);
                let ejk = dist(xy[j], xy[k]);
                let ai =
                    ((eij * eij + eik * eik - ejk * ejk) / (2.0 * eij * eik)).acos();
                let aj =
                    ((eij * eij + ejk * ejk - eik * eik) / (2.0 * eij * ejk)).acos();
                let ak =
                    ((eik * eik + ejk * ejk - eij * eij) / (2.0 * eik * ejk)).acos();
                let bi = (2.0 * ai).sin();
                let bj = (2.0 * aj).sin();
                let bk = (2.0 * ak).sin();
                let x =
                    bi * xy[i].0 as f64 + bj * xy[j].0 as f64 + bk * xy[k].0 as f64;
                let y =
                    bi * xy[i].1 as f64 + bj * xy[j].1 as f64 + bk * xy[k].1 as f64;
                let x = x / (bi + bj + bk);
                let y = y / (bi + bj + bk);
                let dx = x - xy[i].0 as f64;
                let dy = y - xy[i].1 as f64;
                let r = (dx * dx + dy * dy).sqrt();
                if check(&xy, x, y, r) {
                    mi = mi.min(r);
                }
            }
        }
    }
    puts!("{}\n", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
