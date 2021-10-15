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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        tv: [(usize1, i64); q],
    }
    let mut a = a;
    let mut ma = 0;
    let mut cur = 0;
    for i in 0..24 {
        cur += a[i];
    }
    for i in 0..n - 23 {
        ma = max(ma, cur);
        if i + 24 < n {
            cur -= a[i];
            cur += a[i + 24];
        }
    }
    for &(t, v) in &tv {
        a[t] = v;
        let l = max(23, t) - 23;
        let r = min(n - 24, t) + 1;
        let mut cur = 0;
        for i in 0..24 {
            cur += a[l + i];
        }
        for i in l..r {
            ma = max(ma, cur);
            if i + 1 < r {
                cur -= a[i];
                cur += a[i + 24];
            }
        }
        puts!("{}\n", ma);
    }
}
