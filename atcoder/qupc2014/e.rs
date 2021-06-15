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

// ax^2 + bx + c = 0
// None: infinitely many solutions
fn quad(a: i64, b: i64, c: i64) -> Option<Vec<f64>> {
    if a == 0 {
        if b == 0 {
            if c == 0 {
                return None;
            } else {
                return Some(vec![]);
            }
        }
        return Some(vec![-c as f64 / b as f64]);
    }
    let mut ans = vec![];
    if b * b / 4 >= a * c {
        let cent = -b as f64 / 2.0 / a as f64;
        let delta = ((b as f64 * b as f64 - 4.0 * a as f64 * c as f64) as f64).sqrt() / 2.0 / a.abs() as f64;
        ans.push(cent - delta);
        if b * b % 4 != 0 || b * b / 4 > a * c {
            ans.push(cent + delta);
            if ans[0].abs() < ans[1].abs() {
                ans[0] = c as f64 / a as f64 / ans[1];
            } else {
                ans[1] = c as f64 / a as f64 / ans[0];
            }
        }
    }
    Some(ans)
}

// Tags: quadratic-equation
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(x0: i64, y0: i64, vx: i64, vy: i64, vh: i64);
    let sqv = vx * vx + vy * vy;
    let a1 = 2 * (x0 * vx + y0 * vy);
    let a0 = x0 * x0 + y0 * y0;
    let a2 = sqv - vh * vh;
    let sol = quad(a2, a1, a0);
    if let Some(sol) = sol {
        for s in sol {
            if s > -1.0e-7 {
                puts!("{}\n", s);
                return;
            }
        }
        puts!("IMPOSSIBLE\n");
    } else {
        puts!("0\n");
    }
}
