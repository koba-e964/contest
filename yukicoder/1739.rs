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

fn calc(a: i64, b: i64, x: i64, y: i64, h: &[i64]) -> bool {
    let n = h.len();
    let mut a = a;
    let mut q = 0;
    let mut v = vec![];
    let mut s = 0;
    for i in 0..n {
        q += h[i] / x;
        v.push(h[i] % x);
        s += h[i];
    }
    v.sort();
    let r = min(a, q);
    a -= r;
    q -= r;
    s -= r * x;
    if a > 0 {
        assert_eq!(q, 0);
        for i in 0..min(a as usize, n) {
            s -= v[n - 1 - i];
        }
    }
    s <= b * y
}

fn main() {
    input! {
        n: usize, a: i64, b: i64, x: i64, y: i64,
        h: [i64; n],
    }
    let mut pass = 1 << 30;
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let mut hm = h.clone();
        for v in &mut hm {
            *v = max(0, *v - mid);
        }
        if calc(a, b, x, y, &hm) {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
