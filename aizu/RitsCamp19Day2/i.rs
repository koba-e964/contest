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

fn calc_short(n: i64, st: i64, en: i64, q: usize) -> Option<i64> {
    let mut quo: i64 = 0;
    let mut rem = en;
    for _ in 0..q {
        quo *= 2;
        rem *= 2;
        if rem >= n {
            rem -= n;
            quo += 1;
        }
    }
    let diff = (st - rem + n) % n;
    if st < rem {
        quo += 1;
    }
    assert!(quo <= 1 << q);
    if diff < 1 << q {
        Some((diff ^ quo) & ((1 << q) - 1))
    } else {
        None
    }
}

fn calc(n: i64, st: i64, mut en: i64, q: usize) -> Option<(i64, usize)> {
    let mut k = q;
    loop {
        if k >= 62 { k -= 1; continue; }
        if k == 0 || n > 1 << (k - 1) {
            break;
        }
        k -= 1;
    }
    assert!(1 << k <= 2 * n);
    let rest = q - k;
    for _ in 0..rest {
        en *= 2;
        if en >= n {
            en -= n - 1;
        }
    }
    if let Some(ans) = calc_short(n, st, en, k) {
        Some((ans, rest))
    } else {
        None
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: i64,
        q: usize,
        k: i64,
        d: i64,
    }
    if let Some((ans, rest)) = calc(n, k - 1, d - 1, q) {
        for i in 0..q - rest {
            puts!("{}\n", if (ans & 1 << i) == 0 { 0 } else { 1 });
        }
        for _ in 0..rest {
            puts!("0\n");
        }
    } else {
        puts!("-1\n");
    }
}

fn main() {
    solve();
}
