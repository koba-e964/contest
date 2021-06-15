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

fn calc(ab: &[(i64, i64)], mid: i64) -> bool {
    let n = ab.len();
    let s: i64 = ab.iter().map(|&(a, _)| a).sum();
    let mut v = vec![0; n];
    let mut rem = mid;
    let mut t = vec![(0, 0); n];
    for i in 0..n {
        let a = ab[i].0;
        let q = a * mid / s;
        let r = a * mid - q * s;
        rem -= q;
        v[i] = q;
        t[i] = (-r, i);
    }
    t.sort();
    assert!(rem < n as i64);
    for i in 0..rem as usize {
        v[t[i].1] += 1;
    }
    (0..n).all(|i| v[i] >= ab[i].1)
}

// Tags: lie
fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut pass = 1i64 << 49;
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        if calc(&ab, mid) {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let mut mi = pass;
    for i in max(0, pass - (10 * n * n) as i64)..pass {
        if calc(&ab, i) {
            mi = i;
            break;
        }
    }
    println!("{}", mi);
}
