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

// https://creasys.org/rambo/articles/907427cf7cb938c28123
fn ccw((x1, y1): (i64, i64), (x2, y2): (i64, i64), (x3, y3): (i64, i64))
       -> i32 {
    let dx2 = x2 - x1;
    let dy2 = y2 - y1;
    let dx3 = x3 - x1;
    let dy3 = y3 - y1;
    match (dx2 * dy3).cmp(&(dx3 * dy2)) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => {
            if dx2 * dx3 < 0 || dy2 * dy3 < 0 {
                return -1;
            }
            if dx2 * dx2 + dy2 * dy2 < dx3 * dx3 + dy3 * dy3 {
                return 1;
            }
            0
        },
    }
}

fn intersect((p11, p12): ((i64, i64), (i64, i64)), (p21, p22): ((i64, i64), (i64, i64)))
             -> Option<(i64, i64)> {
    if ccw(p11, p12, p21) * ccw(p11, p12, p22) <= 0
        && ccw(p21, p22, p11) * ccw(p21, p22, p12) <= 0 {
            let (a00, a01, a10, a11) = (p12.0 - p11.0, p22.0 - p21.0,
                                        p12.1 - p11.1, p22.1 - p21.1);
            let (v0, v1) = (p21.0 - p11.0, p21.1 - p11.1);
            let (s_num, s_den) = (v0 * a11 - v1 * a01, a00 * a11 - a10 * a01);
            if s_num * a00 % s_den == 0 && s_num * a10 % s_den == 0 {
                Some((p11.0 + s_num * a00 / s_den, p11.1 + s_num * a10 / s_den))
            } else {
                None
            }
        } else {
            None
        }
}

fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x.abs();
    let mut y = y.abs();
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input!{
        n: usize,
        segs: [((i64, i64), (i64, i64)); n],
    }
    let mut hm = HashMap::new();
    for i in 0 .. n {
        for j in 0 .. i {
            if let Some(pt) = intersect(segs[i], segs[j]) {
                *hm.entry(pt).or_insert(0) += 1;
            }
        }
    }
    let mut ans = 0;
    for ((a, b), (c, d)) in segs {
        ans += gcd(c - a, d - b) + 1;
    }
    for (_, f) in hm {
        for i in 1 .. 1001 {
            if i * (i + 1) == 2 * f {
                ans -= i;
                break;
            }
        }
    }
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
