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
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn pow(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x;
    while e > 0 {
        if e % 2 != 0 { sum = sum * cur % m; }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

fn powsum(rat: i64, n: i64, m: i64) -> i64 {
    if n == 0 {
        return 0;
    }
    if n % 2 == 1 {
        let tmp = rat * powsum(rat, n - 1, m);
        return (tmp + 1) % m;
    }
    powsum(rat, n / 2, m) * (pow(rat, n / 2, m) + 1) % m
}

fn calc(rat: i64, a: i64, b: i64, n: i64, m: i64) -> i64 {
    if n == 0 {
        return 0;
    }
    if n % 2 == 1 {
        let tmp = rat * calc(rat, a, b, n - 1, m) % m;
        let tmp2 = a + (n - 1) * b;
        return (tmp + tmp2) % m;
    }
    let tmp = calc(rat, a, b, n / 2, m) * (pow(rat, n / 2, m) + 1) % m;
    let tmp2 = (n / 2) * b % m;
    let tmp2 = tmp2 * powsum(rat, n / 2, m);
    (tmp + tmp2) % m
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        l: i64, a: i64, b: i64, m: i64
    }
    let mut ten = vec![1i64; 19];
    for i in 0..18 {
        ten[i + 1] = ten[i] * 10;
    }
    let mut tot = 0;
    for d in 0..18 {
        let x = max(0, ten[d] - a + b - 1) / b;
        let y = max(0, ten[d + 1] - a + b - 1) / b;
        let y = min(l, y);
        if x < y {
            let radix = pow(10, d as i64 + 1, m);
            tot = tot * pow(radix, y - x, m) % m;
            tot += calc(radix, a + b * x, b, y - x, m);
            tot %= m;
        }
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
