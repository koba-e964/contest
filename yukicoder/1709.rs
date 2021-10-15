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

const MOD: i64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut inv = vec![0; n];
    for i in 0..n {
        inv[p[i]] = i;
    }
    let mut l = inv[0];
    let mut r = inv[0];
    let mut tot = 1;
    for i in 1..n {
        let old = (l, r);
        l = min(l, inv[i]);
        r = max(r, inv[i]);
        if old != (l, r) {
            continue;
        }
        let t = (r + 1 - l) - i;
        tot = tot * t as i64 % MOD;
    }
    println!("{}", tot);
}
