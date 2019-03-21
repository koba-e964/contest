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

fn comp(a: &[usize], b: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut ret = vec![0; n];
    for i in 0..n {
        ret[i] = b[a[i]];
    }
    ret
}

fn inv(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut ret = vec![0; n];
    for i in 0..n {
        ret[a[i]] = i;
    }
    ret
}

fn pow(a: &[usize], mut k: i64) -> Vec<usize> {
    let mut cur = a.to_vec();
    let mut sum: Vec<_> = (0..a.len()).collect();
    while k > 0 {
        if k % 2 == 1 {
            sum = comp(&sum, &cur);
        }
        cur = comp(&cur, &cur);
        k /= 2;
    }
    sum
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        k: i64,
        p: [usize1; n],
        q: [usize1; n],
    }
    let k = k - 1;
    let mut tbl = vec![p, q];
    for i in 2..6 {
        let tmp = comp(&inv(&tbl[i - 2]), &tbl[i - 1]);
        tbl.push(tmp);
    }
    let hop = pow(&comp(&tbl[0], &tbl[3]), k / 6);
    let ans = comp(&comp(&inv(&hop), &tbl[(k % 6) as usize]), &hop);
    for i in 0..n {
        puts!("{}{}", ans[i] + 1, if i == n - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
