#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn f((x, y): (i64, i64), (z, w): (i64, i64)) -> i64 {
    if x > z || y > w { return 0; }
    (z - x + 1) * (w - y + 1)
}
fn g((x, y): (i64, i64), (z, w): (i64, i64)) -> i64 {
    if x > z || y > w { return 0; }
    let mut val = (z - x + 1) * (w - y + 1) + 1;
    if (x + y) % 2 == 1 {
        val -= 1;
    }
    val / 2
}

fn inter((x1, y1): (i64, i64), (x2, y2): (i64, i64),
         (x3, y3): (i64, i64), (x4, y4): (i64, i64)) -> ((i64, i64), (i64, i64))
{
    ((max(x1, x3), max(y1, y3)), (min(x2, x4), min(y2, y4)))
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        t: usize,
        que: [[(i64, i64); 5]; t], 
    }
    for que in que {
        let (n, m) = que[0];
        let p1 = que[1];
        let p2 = que[2];
        let p3 = que[3];
        let p4 = que[4];
        let mut white = g((1, 1), (m, n));
        let a1 = f(p1, p2);
        let (p5, p6) = inter(p1, p2, p3, p4);
        white -= g(p1, p2) - g(p5, p6) + g(p3, p4);
        white += a1 - f(p5, p6);
        puts!("{} {}\n", white, n * m - white);
    }
        
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
