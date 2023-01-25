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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, x: usize,
        ab: [(usize, usize); n],
    }
    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for (a, mut b) in ab {
        let mut v = vec![];
        let mut c = 1;
        while b > 0 {
            let r = min(b, c);
            v.push(r);
            b -= r;
            c *= 2;
        }
        for r in v {
            let mut ep = dp.clone();
            for i in a * r..x + 1 {
                ep[i] |= dp[i - a * r];
            }
            dp = ep;
        }
    }
    println!("{}", if dp[x] { "Yes" } else { "No" });
}

