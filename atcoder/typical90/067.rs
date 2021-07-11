#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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

fn trans(x: Vec<i64>) -> Vec<i64> {
    let mut v = 0;
    let n = x.len();
    for i in 0..n {
        v = 8 * v + x[i];
    }
    let mut ans = vec![];
    while v > 0 {
        let mut r = v % 9;
        if r == 8 {
            r = 5;
        }
        ans.push(r);
        v /= 9;
    }
    ans.reverse();
    ans
}

fn main() {
    input!(n: String, k: usize);
    let mut n: Vec<_> = n.chars().map(|x| (x as u8 - b'0') as i64).collect();
    for _ in 0..k {
        n = trans(n);
    }
    if n.is_empty() {
        print!("0");
    }
    for n in n {
        print!("{}", n);
    }
    println!();
}
