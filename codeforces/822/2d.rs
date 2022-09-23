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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// The author read the editorial before implementing this.
// Tags: greedy
fn calc(k: usize, x: &[i64]) -> bool {
    let n = x.len();
    let mut pos = k;
    let mut left = vec![];
    let mut right = vec![];
    let mut cur = 0;
    let mut mi = 0;
    while pos > 0 {
        cur += x[pos - 1];
        mi = min(mi, cur);
        if cur >= 0 {
            left.push((cur, mi));
            cur = 0;
            mi = 0;
        }
        pos -= 1;
    }
    left.push((cur, mi));
    cur = 0;
    mi = 0;
    pos = k + 1;
    while pos < n {
        cur += x[pos];
        mi = min(mi, cur);
        if cur >= 0 {
            right.push((cur, mi));
            cur = 0;
            mi = 0;
        }
        pos += 1;
    }
    right.push((cur, mi));
    left.reverse();
    right.reverse();
    let mut now = x[k];
    while !left.is_empty() && !right.is_empty() {
        let &(curl, mil) = left.last().unwrap();
        let &(curr, mir) = right.last().unwrap();
        if now + mil < 0 && now + mir < 0 {
            return false;
        }
        if now + mil >= 0 {
            now += curl;
            left.pop();
            continue;
        }
        now += curr;
        right.pop();
    }
    true
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(x: [([i64], i64)]);
    for (mut x, y) in x {
        let k = x[0] as usize - 1;
        x.remove(0);
        x.push(y);
        puts!("{}\n", if calc(k, &x) { "YES" } else { "NO" });
    }
}
