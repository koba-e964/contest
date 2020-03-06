#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn calc(n: usize, xw: &[(usize, i64)], c: i64) -> Vec<i64> {
    if c == 0 {
        let s = xw.iter().map(|&(_, w)| w).sum();
        return vec![s; n];
    }
    let mut sum = vec![0; n + 1];
    let mut sum2 = vec![0; n + 1];
    for &(x, w) in xw {
        let r = w / c;
        let lo = max(0, x as i64 - r);
        let hi = min(n as i64 - 1, x as i64 + r);
        // [lo, x]
        sum[lo as usize] += w - (lo - x as i64).abs() * c;
        sum2[lo as usize + 1] += c;
        sum2[x + 1] -= 2 * c;
        // [x + 1, hi]
        sum2[hi as usize + 1] += c;
        sum[hi as usize + 1] -= w - (hi - x as i64).abs() * c;
    }
    for i in 0..n {
        sum2[i + 1] += sum2[i];
    }
    for i in 0..n {
        sum[i] += sum2[i];
        sum[i + 1] += sum[i];
    }
    sum.pop();
    sum
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, m: usize,
        a: [i64; n],
        xw: [(usize1, i64); m],
    }
    const INF: i64 = 1 << 20;
    let mut pass = INF;
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let w = calc(n, &xw, mid);
        let ok = (0..n).all(|i| a[i] > w[i]);
        if ok {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    puts!("{}\n", if pass >= INF { -1 } else { pass });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
