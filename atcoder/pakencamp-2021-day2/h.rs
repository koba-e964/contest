use std::cmp::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn calc(mut n: i64, p: i64) -> i64 {
    let mut x = 0;
    while n >= p {
        n /= p;
        x += n;
    }
    x
}

fn main() {
    input! {
        m: usize,
        ps: [(i64, i64); m],
    }
    let mut hm = HashMap::new();
    for (p, s) in ps {
        if let Some(&val) = hm.get(&p) {
            if val != s {
                println!("-1");
                return;
            }
        } else {
            hm.insert(p, s);
        }
    }
    let mut l = 1i64;
    let mut r = 1i64;
    for _ in 0..18 { r *= 10; }
    for (p, s) in hm {
        let mut pass = r + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass - fail) / 2 + fail;
            if calc(mid, p) >= s {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        let lo = pass;
        let mut pass = r + 1;
        let mut fail = lo - 1;
        while pass - fail > 1 {
            let mid = (pass - fail) / 2 + fail;
            if calc(mid, p) >= s + 1 {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        let hi = pass;
        if lo >= hi {
            println!("-1");
            return;
        }
        l = max(l, lo);
        r = min(r, hi - 1);
    }
    if l <= r {
        println!("{}", l);
    } else {
        println!("-1");
    }
}
