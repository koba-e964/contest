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

fn div5(s: Vec<i32>) -> (Vec<i32>, i32) {
    let mut rem = 0;
    let mut dig = vec![];
    for &c in &s {
        rem = rem * 10 + c;
        let cur = rem / 5;
        if !dig.is_empty() || cur > 0 {
            dig.push(cur);
        }
        rem %= 5;
    }
    (dig, rem)
}

fn to_5(mut s: Vec<i32>) -> Vec<i32> {
    let mut dig = vec![];
    while !s.is_empty() {
        let (quo, rem) = div5(s);
        s = quo;
        dig.push(rem);
    }
    dig
}

fn factor(mut x: i32) -> Option<Vec<i32>> {
    if x == 0 {
        return None;
    }
    let mut fac = vec![];
    for d in (2..=5).rev() {
        while x % d == 0 {
            x /= d;
            fac.push(d);
        }
    }
    if x == 1 {
        Some(fac)
    } else {
        None
    }
}

// Tags: bigint, arbitrary-precision, marathon
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        s: chars,
    }
    let s: Vec<i32> = s.into_iter().map(|x| (x as u8 - b'0') as _).collect();
    let mut s5 = to_5(s);
    eprintln!("s5 = {:?}", s5);
    // compaction
    let m = s5.len();
    let mut done = m;
    while done > 0 {
        let mut val = 0;
        let mut ma = (0, 0);
        for j in 1..=min(12, done) {
            val = 5 * val + s5[done - j];
            if factor(val).is_some() {
                ma.chmax((j, val));
            }
        }
        for j in done - ma.0..done {
            s5[j] = 0;
        }
        s5[done - ma.0] = ma.1;
        done -= ma.0;
    }
    // output
    let mut ans = "".to_string();
    let mut fst = true;
    for i in 0..s5.len() {
        if s5[i] > 0 {
            if !fst {
                ans += "+";
            }
            fst = false;
            let mut facs = factor(s5[i]).unwrap();
            facs.extend_from_slice(&vec![5; i]);
            if facs.is_empty() {
                ans += "1";
            } else {
                for i in 0..facs.len() {
                    ans += &format!("{}{}", facs[i], if i + 1 == facs.len() {
                        ""
                    } else {
                        "*"
                    });
                }
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
