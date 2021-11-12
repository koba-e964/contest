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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const MOD: i64 = 998_244_353;

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

fn main() {
    input! {
        n: usize,
        b: [i32; n],
    }
    let mut q_dc = 0;
    let mut q_c = 0;
    let mut s = 0;
    for i in 0..n {
        if (i & (n - 1)) == i {
            if b[i] >= 0 {
                s ^= b[i];
            } else {
                q_c += 1;
            }
        } else {
            if b[i] < 0 {
                q_dc += 1;
            }
        }
    }
    if s == 0 && q_c == 0 {
        println!("0");
        return;
    }
    println!("{}", powmod(2, q_dc + max(0, q_c - 1), MOD));
}
