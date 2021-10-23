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
    input!(n: i64, s: i64, t: i64, a: f64, b: f64);
    let mut pass = 0.0;
    let mut fail = b * n as f64;
    for _ in 0..100 {
        let mid = (fail + pass) / 2.0;
        let mut ex = 0.0;
        let d = ((mid + b) / a).ceil() as i64;
        let d = min(d, t);
        ex += (d * (d - 1) / 2) as f64 * a;
        ex += (n - d) as f64 * (mid + b);
        ex /= n as f64;
        if mid < ex {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    eprintln!("avg = {}", pass);
    let d = ((pass + b) / a).floor() as i64;
    if s <= t && t - s <= d {
        println!("{}", a * (t - s) as f64);
    } else {
        println!("{}", pass + b);
    }
}
