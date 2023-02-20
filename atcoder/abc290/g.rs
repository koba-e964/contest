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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        dkx: [(usize, i64, i64); t],
    }
    for (d, k, x) in dkx {
        let mut tbl = vec![0; d + 1];
        tbl[0] = 1;
        for i in 1..d + 1 {
            tbl[i] = tbl[i - 1] * k + 1;
        }
        let mut mi = tbl[d] - 1;
        for i in 0..d + 1 {
            if x > tbl[i] { continue; }
            let mut tmp = 0;
            let mut y = tbl[i] - x;
            for j in (0..i).rev() {
                tmp += y / tbl[j];
                y %= tbl[j];
            }
            if i < d {
                tmp += 1;
            }
            mi = min(mi, tmp);
        }
        puts!("{}\n", mi);
    }
}
