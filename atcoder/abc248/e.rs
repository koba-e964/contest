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

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

type Coord = i64; // the type of coordinates
type P = (Coord, Coord);
type Line = (Coord, Coord, Coord);

fn line((x1, y1): P, (x2, y2): P) -> Line {
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

fn main() {
    input! {
        n: usize, k: usize,
        xy: [(i64, i64); n],
    }
    if k == 1 {
        println!("Infinity");
        return;
    }
    let mut sol = HashSet::new();
    for i in 0..n {
        for j in 0..i {
            let l = line(xy[i], xy[j]);
            let (a, b, c) = l;
            if xy.iter().filter(|&(x, y)| a * x + b * y == c).count() >= k {
                sol.insert(l);
            }
        }
    }
    println!("{}", sol.len());
}
