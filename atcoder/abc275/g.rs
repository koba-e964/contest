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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn calc(abc: &[(f64, f64, f64)], lambda: f64) -> f64 {
    let mut ma = 0.0f64;
    for &(a, b, c) in abc {
        ma = ma.max((c - b * lambda) / a);
    }
    ma + lambda
}

// Ref: https://www.slideshare.net/wata_orz/ss-91375739 (pp. 71-72)
// Similar problems: http://poj.org/problem?id=2595
// Tags: lagrangian-relaxation, ternary-search, dual-of-linear-programming
fn main() {
    input! {
        n: usize,
        abc: [(f64, f64, f64); n],
    }
    let mut lo = 0.0;
    let mut hi = 10.0; // max / min = 10^9 / 10^8 = 10
    // 0.6^32 = 7.96 * 10^-8 < 10^-7
    for _ in 0..32 {
        let mid1 = lo * 0.6 + hi * 0.4;
        let mid2 = lo * 0.4 + hi * 0.6;
        let v1 = calc(&abc, mid1);
        let v2 = calc(&abc, mid2);
        if v1 < v2 {
            hi = mid2;
        } else {
            lo = mid1;
        }
    }
    println!("{}", calc(&abc, lo));
}

