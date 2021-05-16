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

// Check if exists i, r / y = i / n.
fn calc(n: i64, r: i64, y: i64) -> Option<(i64, i64)> {
    if (r * n) % y == 0 {
        let val = r * n / y;
        if 1 <= val && val <= n {
            Some((n, val))
        } else {
            None
        }
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
        s: chars,
    }
    let s: String = s.into_iter().collect();
    let words: Vec<&str> = s.split("/").collect();
    let x: i64 = words[0].parse().unwrap();
    let y: i64 = words[1].parse().unwrap();
    let q = (2 * x + y) / y;
    let r = q * y - 2 * x;
    let mut ans = vec![];
    // Assume q = n + 1.
    if let Some((n, b)) = calc(q - 1, r, 2 * y) {
        ans.push((n, b));
    }
    // Assume q = n.
    if let Some((n, b)) = calc(q, r + y, 2 * y) {
        ans.push((n, b));
    }
    if ans.is_empty() {
        puts!("Impossible\n");
    } else {
        for (n, b) in ans {
            puts!("{} {}\n", n, b);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
