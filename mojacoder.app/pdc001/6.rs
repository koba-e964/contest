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
    input!(a: i64, b: i64);
    const W: usize = 40_000;
    let mut pr = vec![true; W];
    pr[0] = false;
    pr[1] = false;
    for i in 2..W {
        if !pr[i] { continue; }
        for j in 2..(W - 1) / i + 1 {
            pr[i * j] = true;
        }
    }
    let mut ma = 3;
    for p in 2..W {
        if !pr[p] { continue; }
        let p = p as i64;
        let mut e = 4;
        let mut lo = (a - 1) / p / p / p / p;
        let mut hi = b / p / p / p / p;
        while lo < hi {
            ma = max(ma, e);
            lo /= p;
            hi /= p;
            e += 1;
        }
    }
    println!("{}", ma);
}
