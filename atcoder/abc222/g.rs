use std::cmp::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

// min n, k | (10^n - 1) / 9
fn calc(k: i64) -> i64 {
    if k % 2 == 0 || k % 5 == 0 {
        return -1;
    }
    let k = 9 * k;
    const B: usize = 32000;
    let mut hm = HashMap::new();
    let mut ans = k;
    let mut c = 1;
    for _ in 0..B {
        c = 10 * c % k;
    }
    let bc = c;
    c = 1;
    for i in 0..B {
        c = c * bc % k;
        hm.entry(c).or_insert(1i64 << 40).chmin(B as i64 * (i as i64 + 1));
    }
    c = 1;
    for i in 0..B {
        if let Some(&x) = hm.get(&c) {
            ans.chmin(x - i as i64);
        }
        c = c * 10 % k;
    }
    ans
}

fn main() {
    input! {
        t: usize,
        k: [i64; t],
    }
    for k in k {
        let k = if k % 2 == 0 {
            k / 2
        } else {
            k
        };
        println!("{}\n", calc(k));
    }
}
