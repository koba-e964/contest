use std::cmp::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const INF: i64 = 1 << 50;

fn rot(a: usize, x: i64, y: i64) -> (i64, i64) {
    if a == 0 {
        (x, y)
    } else if a == 1 {
        (y, -x)
    } else if a == 2 {
        (-x, -y)
    } else {
        (-y, x)
    }
}

fn calc1(a: usize, x: i64, y: i64) -> i64 {
    let (x, y) = rot(a / 2, x, y);
    let a = a % 2;
    if a == 0 {
        if x >= 0 && y == 0 { x } else { INF }
    } else {
        if x == y && x >= 0 { x } else { INF }
    }
}

fn calc2(a: usize, b: usize, x: i64, y: i64) -> i64 {
    if (b + 8 - a) % 4 == 0 {
        return INF;
    }
    if a % 2 == 1 && b % 2 == 1 {
        if (x + y) % 2 != 0 {
            return INF;
        }
        return calc2(a - 1, b - 1, (x + y) / 2, (-x + y) / 2);
    }
    if a % 2 == 1 {
        return calc2(b, a, x, y);
    }
    let (x, mut y) = rot(a / 2, x, y);
    let mut b = (b + 8 - a) % 8;
    if b >= 4 {
        y = -y;
        b = 8 - b;
    }
    assert!(1 <= b && b <= 3);
    if b == 2 {
        return if x >= 0 && y >= 0 {
            x + y
        } else {
            INF
        };
    }
    if b == 1 {
        return if y >= 0 && x >= y {
            x
        } else {
            INF
        };
    }
    if y >= 0 && x >= -y {
        x + 2 * y
    } else {
        INF
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        abs: [(i64, i64, chars); t],
    }
    for (a, b, s) in abs {
        let mut mi = if a == 0 && b == 0 { 0 } else { INF };
        for i in 0..8 {
            if s[i] != '1' { continue; }
            mi = min(mi, calc1(i, a, b));
            for j in 0..i {
                if s[j] != '1' { continue; }
                mi = min(mi, calc2(i, j, a, b));
            }
        }
        // TODO: 3 directions
        if (a + b) % 2 != 0 {
            for i in 0..4 {
                if s[2 * i + 1] != '1' { continue; }
                for j in 0..i {
                    if s[2 * j + 1] != '1' { continue; }
                    for k in 0..4 {
                        if s[2 * k] != '1' { continue; }
                        let (mut x, mut y) = (a, b);
                        match k {
                            0 => x -= 1,
                            2 => x += 1,
                            1 => y -= 1,
                            3 => y += 1,
                            _ => panic!(),
                        }
                        mi = min(mi, calc2(2 * i + 1, 2 * j + 1, x, y) + 1);
                    }
                }
            }
        }
        puts!("{}\n", if mi >= INF { -1 } else { mi });
    }
}
