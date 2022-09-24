use std::cmp::*;
use std::io::{Write, BufWriter};
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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, k: i64,
        a: [i64; n],
    }
    let mut pass = 0;
    let mut fail = 1i64 << 40;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut tot = 0;
        for i in 0..n {
            tot += min(mid, a[i]);
        }
        if tot <= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let mut tot = 0;
    let mut a = a;
    for i in 0..n {
        let r = min(pass, a[i]);
        tot += r;
        a[i] -= r;
    }
    let mut rem = k - tot;
    for i in 0..n {
        if a[i] > 0 && rem > 0 {
            a[i] -= 1;
            rem -= 1;
        }
    }
    putvec!(a);
}
