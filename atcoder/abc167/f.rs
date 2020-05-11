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

// Tags: parentheses non-standard-sort string-sort
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        s: [chars; n],
    }
    // (delta, min)
    let s: Vec<(i64, i64)> = s.into_iter().map(|s| {
        let mut delta = 0;
        let mut mi = 0;
        for c in s {
            if c == '(' {
                delta += 1;
            } else {
                delta -= 1;
            }
            mi = min(mi, delta);
        }
        (delta, mi)
    }).collect();
    let mut delta = 0;
    let mut pos = vec![];
    let mut neg = vec![];
    for (d, m) in s {
        if m == 0 {
            delta += d;
        } else if d > 0 {
            pos.push((d, m));
        } else {
            neg.push((d, m));
        }
    }
    pos.sort_by_key(|&(_d, m)| -m);
    neg.sort_by_key(|&(d, m)| -(d - m));
    let mut mi = delta;
    for &(d, m) in &pos {
        mi = min(mi, delta + m);
        delta += d;
    }
    for &(d, m) in &neg {
        mi = min(mi, delta + m);
        delta += d;
    }
    puts!("{}\n", if delta == 0 && mi >= 0 { "Yes" } else { "No" });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
