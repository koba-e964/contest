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

pub fn naive(r: i64) -> i64 {
    assert!(r <= 1000);
    let mut ans = 0;
    let mut seen = vec![vec![false; r as usize + 1]; r as usize + 1];
    for x in 1..r + 1 {
        for z in x..r + 1 {
            let y = (((z * z - x * x) as f64).sqrt() + 0.5).floor();
            seen[x as usize][y as usize] = true;
            seen[y as usize][x as usize] = true;
        }
    }
    for x in 1..r + 1 {
        let y = ((r * r + r - x * x) as f64 + 0.25).sqrt().ceil() as usize;
        for i in 1..min(r as usize + 1, y) {
            if !seen[x as usize][i] {
                ans += 1;
            }
        }
    }
    4 * ans
}

pub fn precomp(n: usize) -> Vec<i64> {
    let mut tbl = vec![0; n];
    for i in 2..n {
        let targ = ((i as f64 * i as f64 - 0.125) as f64 / 2.0).sqrt() + 0.125;
        let targ = targ.floor() as i64;
        let y = (((i as i64 * i as i64 - targ * targ) as f64).sqrt() + 0.5).floor() as i64;
        let me = if targ != y {
            2 * targ
        } else {
            2 * targ - 1
        };
        if i < 10 {
            eprintln!("{} => {}", i, me);
        }
        tbl[i] = tbl[i - 1] + me;
    }
    tbl
}

fn max_y(r: i64, x: i64) -> i64 {
    min(r, ((r * r + r - x * x) as f64 + 0.25).sqrt().floor() as i64)
}

pub fn calc(r: i64, tbl: &[i64]) -> i64 {
    let mut ans = 0;
    let lim = ((r as f64 + 0.5) / (2.0f64).sqrt()).ceil() as i64;
    for x in 1..lim + 1 {
        let this_r = (((x as f64 * x as f64 - 0.125) as f64 / 2.0).sqrt() - 0.25) as i64;
        if this_r > r { continue; }
        let this_y = max_y(this_r, x);
        let this = this_r - this_y;
        let next_r = (((x as f64 * x as f64 - 0.125) as f64 / 2.0).sqrt() + 0.25) as i64;
        if next_r <= r {
            let next_y = max_y(next_r, x); 
            let next = next_r - next_y;
            let last_y = max_y(r, x);
            let last = r - last_y;
            eprintln!("r = {}, x = {}, this_y = {}, last_y = {} , this = {}, next = {}, last = {}", r, x, this_y, last_y, this, next, last);
            ans += 2 * (next - last);
            ans += this - next;
        }
    }
    4 * ans
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
    input! {
        t: usize,
        r: [i64; t],
    }
    let tbl = precomp(100_001);
    for i in 0..t {
        let r = r[i];
        eprintln!("{}",  naive(r));
        puts!("Case #{}: {}\n", i + 1, calc(r, &tbl));
    }
}
