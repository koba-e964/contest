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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        s: [i64; n],
        t: [i64; n],
    }
    let mut s: Vec<_> =
        s.into_iter().enumerate().map(|(i, s)| (s, i)).collect();
    let mut t = t;
    s.sort();
    t.sort();
    let mut op = vec![];
    let mut pos = vec![];
    let mut neg = vec![];
    let mut d = 0;
    for i in 0..n {
        let diff = t[i] - s[i].0;
        if diff > 0 {
            pos.push((diff, s[i].1));
        }
        if diff < 0 {
            neg.push((diff, s[i].1));
        }
        d += diff;
        if d < 0 {
            puts!("NO\n");
            return;
        }
    }
    if d != 0 {
        puts!("NO\n");
        return;
    }
    while let (Some(mut x), Some(mut y)) = (pos.pop(), neg.pop()) {
        let m = min(x.0, -y.0);
        x.0 -= m;
        y.0 += m;
        op.push((x.1, y.1, m));
        if x.0 > 0 {
            pos.push(x);
        }
        if y.0 < 0 {
            neg.push(y);
        }
    }
    puts!("YES\n{}\n", op.len());
    for &(a, b, c) in &op {
        puts!("{} {} {}\n", a + 1, b + 1, c);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
