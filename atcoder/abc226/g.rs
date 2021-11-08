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

fn calc(mut a: Vec<i64>, mut b: Vec<i64>) -> bool {
    if a[4] > b[4] {
        return false;
    }
    b[4] -= a[4];
    a[4] = 0;
    if a[3] > b[3] + b[4] {
        return false;
    }
    let x = min(a[3], b[3]);
    b[3] -= x;
    a[3] -= x;
    let x = min(a[3], b[4]);
    b[4] -= x;
    a[3] -= x;
    b[0] += x;
    for i in 1..3 {
        let x = min(a[i], b[i]);
        a[i] -= x;
        b[i] -= x;
    }
    if a[1] >= 0 && a[2] == 0 {
        if a[1] > b[2] + 2 * b[3] + 2 * b[4] {
            return false;
        }
        return a[0] + 2 * a[1] <= b[0] + 2 * b[1] + 3 * b[2] + 4 * b[3] + 5 * b[4];
    }
    if a[1] == 0 && a[2] > 0 {
        if a[2] > b[3] + b[4] {
            return false;
        }
        return a[0] + 3 * a[2] <= b[0] + 2 * b[1] + 3 * b[2] + 4 * b[3] + 5 * b[4];
    }
    let x = min(min(a[1], a[2]), b[4]);
    b[4] -= x;
    a[1] -= x;
    a[2] -= x;
    if a[1] >= 0 && a[2] == 0 {
        if a[1] > b[2] + 2 * b[3] + 2 * b[4] {
            return false;
        }
        return a[0] + 2 * a[1] <= b[0] + 2 * b[1] + 3 * b[2] + 4 * b[3] + 5 * b[4];
    }
    if a[1] == 0 && a[2] > 0 {
        if a[2] > b[3] + b[4] {
            return false;
        }
        return a[0] + 3 * a[2] <= b[0] + 2 * b[1] + 3 * b[2] + 4 * b[3] + 5 * b[4];
    }
    assert_eq!(b[4], 0);
    if (a[1] + 1) / 2 + a[2] > b[3] {
        return false;
    }
    a[0] + 2 * a[1] + 3 * a[2] <= b[0] + 2 * b[1] + 3 * b[2] + 4 * b[3] + 5 * b[4]
}

// Tags: implementation, ad-hoc
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        ab: [([i64; 5], [i64; 5]); t],
    }
    for (a, b) in ab {
        puts!("{}\n", if calc(a, b) { "Yes" } else { "No" });
    }
}
