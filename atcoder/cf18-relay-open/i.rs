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

// The author read the answer of other contestants before implementing this.
// Tags: confluence
// https://kmjp.hatenablog.jp/entry/2018/11/22/0900
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        t: [usize; n],
    }
    let mut freq = [0; 5];
    for &v in &t {
        freq[v % 5] += 1;
    }
    macro_rules! contr {
        ($a:expr, $b:expr) => {
            let r = min(freq[$a], freq[$b]);
            freq[$a] -= r;
            freq[$b] -= r;
            freq[($a + $b) % 5] += r;
        }
    }
    loop {
        let oldfreq = freq;
        // 3 + 4 ==> 2
        contr!(3, 4);
        // 4 + 4 + 4 ==> 2
        let q = freq[4] / 3;
        freq[4] -= 3 * q;
        freq[2] += q;
        // 4 + 4 --> 3
        let q = freq[4] / 2;
        freq[4] -= 2 * q;
        freq[3] += q;
        // 3 + 3 ==> 1
        let q = freq[3] / 2;
        freq[3] -= 2 * q;
        freq[1] += q;
        // 2 + 3 --> 0
        contr!(2, 3);
        // 2 + 4 --> 1
        contr!(2, 4);
        // 1 + 3 --> 4
        contr!(1, 3);
        // 1 + 4 --> 0
        contr!(1, 4);
        // 1 + 1 --> 2
        let q = freq[1] / 2;
        freq[1] -= 2 * q;
        freq[2] += q;
        if oldfreq == freq {
            break;
        }
    }
    let mut sum_val = 0;
    let mut count = 0;
    for i in 0..n {
        sum_val += t[i];
    }
    for i in 1..5 {
        count += freq[i];
    }
    if count == 0 {
        count = 1;
    }
    sum_val -= freq[1] + 2 * freq[2];
    sum_val += 2 * freq[3] + freq[4];
    puts!("{} {}\n", sum_val, count);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
