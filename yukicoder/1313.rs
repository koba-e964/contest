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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, k: usize,
        s: chars,
    }
    let mut acc = vec![0; n + 1];
    let mut dp = vec![0; n];
    for i in (0..n).rev() {
        if i > 0 && s[i - 1] == 'x' {
            dp[i] = 1;
        } else {
            let hi = min(n, i + k + 1);
            dp[i] = if acc[i + 1] - acc[hi] == hi - i - 1 {
                0
            } else {
                1
            };
        }
        acc[i] = acc[i + 1] + dp[i];
    }
    if dp[0] == 0 {
        puts!("0\n");
    } else {
        for i in 1..k + 1 {
            if dp[i] == 0 {
                puts!("{}\n", i);
            }
        }
    }
}
