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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn calc(p: &[i64], ab: &[(i64, i64)], cd: &[(i64, i64)]) -> i64 {
    let mut hm = HashMap::new();
    for i in 0..p.len() {
        let p = p[i];
        let (a, b) = ab[i];
        for &(c, d) in cd {
            *hm.entry((a - c, b - d)).or_insert(0) += p;
        }
    }
    *hm.values().max().unwrap()
}

fn main() {
    input! {
        n: usize, m: usize,
        p: [i64; n],
        ab: [(i64, i64); n],
        cd: [(i64, i64); m],
    }
    let mut cd = cd;
    let mut ma = 0;
    for _ in 0..2 {
        for _ in 0..2 {
            ma = max(ma, calc(&p, &ab, &cd));
            for v in &mut cd {
                v.0 *= -1;
            }
        }
        for v in &mut cd {
            v.1 *= -1;
        }
    }
    println!("{}", ma);
}
