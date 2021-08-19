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

const INF: i64 = 1 << 50;

fn calc(aa: &[usize], usable: u32) -> i64 {
    let m = aa.len();
    let mut last = [0, INF];
    for i in 1..10 {
        if (usable & 1 << i) != 0 {
            last[1] = i;
            break;
        }
    }
    for i in 0..m {
        let r = aa[i];
        let mut me = [INF; 2];
        for b in 0..2 {
            for j in 0..10 {
                if (usable & 1 << j) == 0 {
                    continue;
                }
                let nxt = if j < r + b {
                    1
                } else {
                    0
                };
                me[b] = min(me[b], 10 * last[nxt] + j as i64);
            }
        }
        last = me;
    }
    min(last[0], last[1])
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(nk: [(i64, u32)]);
    for (n, k) in nk {
        let mut mi = INF;
        let mut aa = vec![];
        let mut v = n;
        while v > 0 {
            aa.push((v % 10) as usize);
            v /= 10;
        }
        aa.reverse();
        for bits in 0u32..1 << 10 {
            if bits.count_ones() != k {
                continue;
            }
            mi.chmin(calc(&aa, bits));
        }
        puts!("{}\n", mi);
    }
}
