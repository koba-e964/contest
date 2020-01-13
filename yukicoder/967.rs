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

const INF: i64 = 1 << 62;

fn mv(
    (a, b, c): (i64, i64, i64),
    (d, e, f): (i64, i64, i64),
    x: i64, y: i64, z: i64,
) -> i64 {
    if a < d || b < e || c < f {
        return INF;
    }
    if d <= 0 || e <= 0 || f <= 0 {
        return INF;
    }
    if d == f {
        return INF;
    }
    if (d - e) * (f - e) <= 0 {
        return INF;
    }
    (a - d) * x + (b - e) * y + (c - f) * z
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        abcxyz: [(i64, i64, i64, i64, i64, i64)],
    }
    for (a, b, c, x, y, z) in abcxyz {
        let k = (a, b, c);
        let mut mi = INF;
        mi = min(mi, mv(k, k, x, y, z));
        // a < b > c
        {
            let d = min(a, b - 1);
            let f = min(c, b - 1);
            if d == f {
                mi = min(mi, mv(k, (d - 1, b, f), x, y, z));
                mi = min(mi, mv(k, (d, b, f - 1), x, y, z));
            } else {
                mi = min(mi, mv(k, (d, b, f), x, y, z));
            }
        }
        // a > b < c
        {
            let d = if a == c { a - 1 } else { a };
            let e = min(b, min(d, c) - 1);
            mi = min(mi, mv(k, (d, e, c), x, y, z));
            let f = if a == c { c - 1 } else { c };
            let e = min(b, min(a, f) - 1);
            mi = min(mi, mv(k, (a, e, f), x, y, z));
        }
        puts!("{}\n", if mi < INF { mi } else { -1 });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
