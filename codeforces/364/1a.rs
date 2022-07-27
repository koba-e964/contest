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

fn main() {
    input!(n: usize, l: f64, v1: f64, v2: f64, k: usize);
    let mut pass = l / v1;
    let mut fail = l / v2;
    for _ in 0..50 {
        let mid = (pass + fail) / 2.0;
        let mut n = n;
        let mut t = 0.0;
        let mut ok = true;
        while n > 0 {
            let x = v1 * v2 / (v1 - v2) * (mid - t + v1 * t / v2 - l / v1);
            if x > l {
                ok = false;
                break;
            }
            let dist = x - v1 * t;
            t += dist / v2 * (1.0 + (v2 - v1) / (v1 + v2));
            n = max(n, k) - k;
        }
        if ok {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
