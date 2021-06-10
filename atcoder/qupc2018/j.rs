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

fn diff(s: &[char]) -> (Vec<i32>, i32) {
    let n = s.len();
    let mut dp = vec![0; n];
    let mut last = 'a';
    for i in 0..n {
        if last != s[i] {
            dp[i] = 1;
        }
        last = s[i];
    }
    (dp, if last == 'b' { 1 } else { 0 })
}

fn get(s: &[u64], idx: usize, len: usize) -> u64 {
    debug_assert!(len <= 64);
    let q = idx / 64;
    let r = idx % 64;
    let mut val = 0;
    if r > 0 {
        if q < s.len() {
            val = s[q] >> r;
        }
        if q + 1 < s.len() {
            val |= s[q + 1] << (64 - r);
        }
    } else {
        val = s[q];
    }
    if len < 64 {
        val &= (1 << len) - 1;
    }
    val
}

// Tags: sqrt-decomposition, bitwise-operations, bitset, constant-factor-optimization
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        s: chars,
        t: [chars],
    }
    let (s, _) = diff(&s);
    const B: usize = 60;
    let mut zeros = vec![vec![]; B];
    let mut ones = vec![vec![]; B];
    let n = s.len();
    for i in 1..B {
        zeros[i] = vec![0; i];
        ones[i] = vec![0; i];
        for j in 0..n {
            if s[j] == 0 {
                zeros[i][j % i] += 1;
            } else {
                ones[i][j % i] += 1;
            }
        }
    }
    // bitmask
    let q = (n + 63) / 64;
    let mut bit = vec![0u64; q];
    for i in 0..n {
        if s[i] == 1 {
            bit[i / 64] |= 1 << i;
        }
    }
    for t in t {
        let (mut t, odd) = diff(&t);
        t[0] ^= odd;
        let mut tot = 0;
        if t.len() < B {
            for i in 0..t.len() {
                if t[i] == 0 {
                    tot += ones[t.len()][i];
                } else {
                    tot += zeros[t.len()][i];
                }
            }
        } else {
            let m = t.len();
            let mut this = vec![0u64; (m + 63) / 64];
            for i in 0..m {
                if t[i] == 1 {
                    this[i / 64] |= 1 << i;
                }
            }
            for i in 0..(n + m - 1) / m {
                let rem = min(n, i * m + m) - i * m;
                for j in 0..(rem + 63) / 64 {
                    let len = min(rem, 64 * j + 64) - 64 * j;
                    let mask = get(&bit, i * m + 64 * j, len);
                    let me = get(&this, 64 * j, len);
                    tot += (mask ^ me).count_ones() as i64;
                }
            }
        }
        if odd == 1 {
            if s[0] == t[0] {
                // 0 -> 1
                tot += 1;
            } else {
                // 1 -> 0
                tot -= 1;
            }
        }
        puts!("{}\n", (tot + 1) / 2);
    }
}
