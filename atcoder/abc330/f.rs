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

fn f(x: &[i64], d: i64) -> i64 {
    let mut ev = x.to_vec();
    for &x in x {
        ev.push(x + d);
    }
    ev.sort_unstable();
    let y = ev[x.len()];
    let mut sum = 0;
    for &x in x {
        sum += max(x - y, max(0, y - x - d));
    }
    sum
}

// Tags: convex-optimization
fn main() {
    input! {
        n: usize, k: i64,
        xy: [(i64, i64); n],
    }
    let mut x = vec![0; n];
    let mut y = vec![0; n];
    for i in 0..n {
        x[i] = xy[i].0;
        y[i] = xy[i].1;
    }
    let mut pass = 1 << 30;
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        if f(&x, mid) + f(&y, mid) <= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
