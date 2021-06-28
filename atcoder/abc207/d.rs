#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
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

fn align(x: &mut [(f64, f64)]) {
    let n = x.len() as f64;
    let mut a = 0.0;
    let mut b = 0.0;
    for &(x, y) in x.iter() {
        a += x;
        b += y;
    }
    a /= n;
    b /= n;
    for p in x {
        p.0 -= a;
        p.1 -= b;
    }
}

fn main() {
    input! {
        n: usize,
        x: [(f64, f64); n],
        y: [(f64, f64); n],
    }
    if n == 1 {
        println!("Yes");
        return;
    }
    let norm = |(x, y)| x * x + y * y;
    let arg = |(x, y): (f64, f64)| y.atan2(x);
    let rot = |(x, y), theta: f64| {
        let c = theta.cos();
        let s = theta.sin();
        (x * c - y * s, x * s + y * c)
    };
    let mut x = x;
    let mut y = y;
    align(&mut x);
    align(&mut y);
    let mut idx = 0;
    for i in 0..n {
        if norm(x[i]) > 1e-9 {
            idx = i;
            break;
        }
    }
    for i in 0..n {
        if (norm(x[idx]) - norm(y[i])).abs() < 1e-9 {
            let theta = arg(y[i]) - arg(x[idx]);
            let mut z = vec![(0.0, 0.0); n];
            for j in 0..n {
                z[j] = rot(x[j], theta);
            }
            const EPS: f64 = 1e-9;
            let ok = (0..n).all(|i| {
                (0..n).any(|j|
                           (z[i].0 - y[j].0).abs() < EPS && (z[i].1 - y[j].1).abs() < EPS
                )
            });
            if ok {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
