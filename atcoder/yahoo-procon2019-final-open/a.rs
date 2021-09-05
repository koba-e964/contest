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

fn calc(h: i64, a: i64) -> f64 {
    let mut tot = 0.0;
    // [0, a] /\ [0, h - a]
    let lim = min(h - a, a);
    tot += (a * lim * lim) as f64 / 2.0 - (lim * lim * lim) as f64 / 6.0;
    // [0, h - a] - [0, a]
    tot += (a * a) as f64 / 2.0 * (h - a - lim) as f64;
    tot as f64 / (h - a) as f64 / (h - a) as f64 * 2.0
}

fn main() {
    input!(h: i64, w: i64, a: i64, b: i64);
    println!("{}", calc(h, a) * calc(w, b));
}
