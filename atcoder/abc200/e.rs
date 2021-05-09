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

fn calc(n: i64, x: i64) -> i64 {
    if !(0 <= x && x <= 3 * n - 3) {
        return if x > 3 * n - 3 { n * n * n } else { 0 };
    }
    if x < n {
        let x = x + 1;
        return x * (x + 1) * (x + 2) / 6;
    }
    if 3 * n - 3 - x < n {
        let y = 3 * n - 3 - x;
        return n * n * n - y * (y + 1) * (y + 2) / 6;
    }
    if x <= 2 * (n - 1) {
        let t = x - n + 1;
        let x = x + 1;
        let mut s = x * (x + 1) * (x + 2) / 6;
        s -= t * (t + 1) * (t + 2) / 2;
        return s;
    }
    panic!();
}

fn calc2(n: i64, x: i64) -> i64 {
    if !(0 <= x && x <= 2 * n - 2) {
        return 0;
    }
    if x < n {
        return x + 1;
    }
    2 * n - 1 - x
}

// Tags: cube
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
        n: usize, k: i64,
    }
    /*
    for i in 0..3 * n {
        eprintln!("{} {} => {}", n, i, calc(n as i64, i as i64));
}*/

    let mut pass = -1;
    let mut fail = 3 * n as i64 - 3;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        if calc(n as i64, mid) >= k {
            fail = mid;
        } else {
            pass = mid;
        }
    }
    let sum = fail;
    let idx = k - calc(n as i64, pass) - 1;
    eprintln!("{} {}th", sum, idx);
    let mut rem = idx;
    let mut cur = 0;
    let mut ans = (0, 0);
    while rem >= 0 {
        let res = sum - cur;
        // eprintln!("cur = {}, res = {}, rem = {}", cur, res, rem);
        let x = calc2(n as i64, res);
        if rem >= x {
            rem -= x;
            cur += 1;
            continue;
        }
        if res >= n as i64 - 1 {
            let y = res - n as i64 + 1;
            ans = (y + rem, n as i64 - 1 - rem);
        } else {
            ans = (rem, res - rem);
        }
        break;
    }
    puts!("{} {} {}\n", cur + 1, ans.0 + 1, ans.1 + 1);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
