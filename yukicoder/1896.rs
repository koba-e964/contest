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

// https://yukicoder.me/problems/no/1896 (3)
// A, B の中で一番高いビットを h とする。A の中に h が立っている数がなければ、B の数から作る。(高々 1 回)
// その後、B の全ての数で h が立っているようにする。(高々 N 回)
// その後、A の全ての数で h が立っていないようにする。(高々 N 回)
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [u32; n],
        b: [u32; n],
    }
    let mut a = a;
    let mut b = b;
    let mut ma = 0;
    for i in 0..n {
        ma = max(ma, max(a[i], b[i]));
    }
    let mut h = 1;
    while h <= ma { h *= 2; }
    h /= 2;
    let mut ops = vec![];
    let mut idx = n;
    for i in 0..n {
        if (a[i] & h) != 0 {
            idx = i;
            break;
        }
    }
    if idx == n {
        for i in 0..n {
            if (b[i] & h) != 0 {
                idx = i;
                break;
            }
        }
        a[0] ^= b[idx];
        ops.push((1, 0, idx));
        idx = 0;
    }
    for i in 0..n {
        if (b[i] & h) == 0 {
            b[i] ^= a[idx];
            ops.push((2, idx, i));
        }
    }
    for i in 0..n {
        if (a[i] & h) != 0 {
            a[i] ^= b[0];
            ops.push((1, i, 0));
        }
    }
    puts!("{}\n", ops.len());
    for (a, b, c) in ops {
        puts!("{} {} {}\n", a, b + 1, c + 1);
    }
}
