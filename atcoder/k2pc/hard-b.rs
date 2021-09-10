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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: union-of-segments, union-of-intervals
fn main() {
    input! {
        k: usize,
        n: usize,
        p: [(usize, i64); n],
    }
    let mut a = vec![];
    for (x, y) in p {
        let y = (y - 1) << (k - x);
        let mut sum = 0;
        for i in 0..k {
            if (y & 1 << i) != 0 {
                sum += (1 << (i + 1)) - 1;
            }
        }
        let len = (1i64 << (k - x + 1)) - 1;
        a.push((sum, sum + len));
    }
    a.sort();
    let mut tot = 0;
    let mut last = 0;
    for (x, y) in a {
        tot += max(0, y - max(last, x));
        last = max(last, y);
    }
    println!("{}", (1 << (k + 1)) - 1 - tot);
}
