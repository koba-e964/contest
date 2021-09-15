use std::cmp::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn nth(a: i64, n: i64) -> i64 {
    let mut pass = 0;
    let mut fail = min(a, 1 << ((60 + n - 1) / n)) + 1;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut tmp = 1i64;
        for _ in 0..n {
            tmp = tmp.saturating_mul(mid);
        }
        if tmp <= a {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    pass
}

fn main() {
    input!(n: i64);
    let mut ans = n;
    for b in 2..60 {
        let x = nth(n, b);
        let mut tmp = 1;
        for _ in 0..b {
            tmp *= x;
        }
        ans = min(ans, n - tmp + x + b);
    }
    println!("{}", ans);
}
