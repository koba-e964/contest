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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        k: usize,
        s: chars,
        q: usize,
        pc: [(usize1, char); q]
    }
    let mut poss = vec![vec![]; k + 1];
    let mut res = vec![vec![]; k];
    for i in 0..k {
        res[i] = s[(1 << k) - (1 << (k - i))..(1 << k) - (1 << (k - i - 1))]
            .to_vec();
        poss[i + 1] = vec![0; res[i].len()];
    }
    poss[0] = vec![1; 1 << k];
    for i in 0..k {
        for j in 0..res[i].len() {
            let val = match res[i][j] {
                '0' => poss[i][2 * j],
                '1' => poss[i][2 * j + 1],
                '?' => poss[i][2 * j] + poss[i][2 * j + 1],
                _ => panic!(),
            };
            poss[i + 1][j] = val;
        }
    }
    for &(p, c) in &pc {
        let mut lv = 0;
        for i in 0..k {
            if (1 << k) - (1 << (k - i)) <= p {
                lv = i;
            }
        }
        let mut idx = p - ((1 << k) - (1 << (k - lv)));
        res[lv][idx] = c;
        while lv < k {
            let val = match res[lv][idx] {
                '0' => poss[lv][2 * idx],
                '1' => poss[lv][2 * idx + 1],
                '?' => poss[lv][2 * idx] + poss[lv][2 * idx + 1],
                _ => panic!(),
            };
            poss[lv + 1][idx] = val;
            lv += 1;
            idx /= 2;
        }
        puts!("{}\n", poss[k][0]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
