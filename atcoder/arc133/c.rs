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

fn sum(a: &[i64], k: i64) -> i64 {
    let mut s = 0;
    for &v in a {
        s += v;
        if s >= k {
            s -= k;
        }
    }
    s
}

fn main() {
    input! {
        h: usize, w: usize, k: i64,
        a: [i64; h],
        b: [i64; w],
    }
    if sum(&a, k) != sum(&b, k) {
        println!("-1");
        return;
    }
    let mut a = a;
    let mut b = b;
    for i in 0..h {
        a[i] = (2 * k - w as i64 % k - a[i]) % k;
    }
    for i in 0..w {
        b[i] = (2 * k - h as i64 % k - b[i]) % k;
    }
    let sa: i64 = a.iter().sum();
    let sb: i64 = b.iter().sum();
    println!("{}", (k - 1) * h as i64 * w as i64 - max(sa, sb));
}
