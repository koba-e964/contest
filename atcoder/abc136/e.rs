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

fn calc(a: &[i64], x: i64) -> i64 {
    let n = a.len();
    let mut rem = vec![0; n];
    let mut pos = vec![];
    let mut neg = vec![];
    let mut possum = 0;
    let mut negsum = 0;
    for i in 0..n {
        rem[i] = a[i] % x;
        if x - rem[i] < rem[i] {
            rem[i] -= x;
        }
        if rem[i] > 0 {
            pos.push(rem[i]);
            possum += rem[i];
        }
        if rem[i] < 0 {
            neg.push(rem[i]);
            negsum += rem[i];
        }
    }
    pos.sort();
    pos.reverse();
    neg.sort();
    if possum + negsum == 0 {
        return possum;
    }
    if possum + negsum > 0 {
        let iter = (possum + negsum) / x;
        for i in 0..iter {
            possum -= pos[i as usize];
        }
        return possum;
    }
    let iter = -(possum + negsum) / x;
    for i in 0..iter {
        negsum -= neg[i as usize];
    }
    -negsum
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, k: i64,
        a: [i64; n],
    }
    let sum: i64 = a.iter().sum();
    const W: i64 = 80000;
    let mut factor = vec![];
    for i in 1..W {
        if sum % i == 0 {
            factor.push(i);
            if sum != i * i {
                factor.push(sum / i);
            }
        }
    }
    factor.sort();
    factor.reverse();
    for &x in &factor {
        if calc(&a, x) <= k {
            puts!("{}\n", x);
            return;
        }
    }
    unreachable!();
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
