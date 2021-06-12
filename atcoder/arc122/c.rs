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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn gcd_pre(x: i64, y: i64) -> i64 {
    if x == 0 {
        return y;
    }
    if y == 0 {
        return x;
    }
    if x <= y {
        let q = y / x;
        let g = gcd_pre(x, y - q * x);
        return g + q;
    }
    if x > y {
        let q = x / y;
        let g = gcd_pre(x - q * y, y);
        return g + q;
    }
    unreachable!();
}

fn gcd(x: i64, y: i64, ops: &mut Vec<i32>) {
    if x == 0 {
        for _ in 0..y {
            ops.push(2);
        }
        return;
    }
    if y == 0 {
        for _ in 0..x {
            ops.push(1);
        }
        return;
    }
    if x <= y {
        let q = y / x;
        let g = gcd(x, y - q * x, ops);
        for _ in 0..q {
            ops.push(4);
        }
        return g;
    }
    if x > y {
        let q = x / y;
        let g = gcd(x - q * y, y, ops);
        for _ in 0..q {
            ops.push(3);
        }
        return g;
    }
    unreachable!();
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input!(n: i64);
    let nn = n as i128;
    let mut pass = 0i128;
    let mut fail = nn;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        if mid * mid + mid * nn <= nn * nn {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let x = pass as i64;
    eprintln!("n = {}, x = {}", n, x);
    let mut cand = -1;
    for i in max(x - 100, 0)..x + 100 {
        let c = gcd_pre(n, i);
        if c <= 130 {
            cand = i;
            break;
        }
    }
    assert!(cand >= 0);
    let mut ops = vec![];
    gcd(n, cand, &mut ops);
    eprintln!("ops.len() == {}", ops.len());
    assert!(ops.len() <= 130);
    puts!("{}\n", ops.len());
    for i in 0..ops.len() {
        puts!("{}\n", ops[i]);
    }
}
