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

fn calc(d: usize, freq: &[Vec<usize>], x: usize) -> bool {
    let (q, r) = (x / d, x % d);
    let mut acc = vec![vec![0i64; 2 * d + 1]; 2 * d + 1];
    for i in 0..2 * d {
        for j in 0..2 * d {
            if freq[i % d][j % d] > (q + 1) * (q + 1) {
                return false;
            }
            let truth = if freq[i % d][j % d] <= q * q {
                1
            } else if freq[i % d][j % d] <= q * (q + 1) {
                0
            } else {
                -1 << 30
            };
            acc[i + 1][j + 1] = acc[i][j + 1] + acc[i + 1][j]
                + truth - acc[i][j];
        }
    }
    let count = |(x0, y0): (usize, usize), (x1, y1): (usize, usize)|
    acc[x1][y1] - acc[x1][y0] - acc[x0][y1] + acc[x0][y0];
    for i in 0..d {
        for j in 0..d {
            if count((i + r, j + r), (i + d, j + d)) != ((d - r) * (d - r)) as i64 {
                continue;
            }
            if count((i + r, j), (i + d, j + d))
                + count((i, j + r), (i + d, j + d)) < 0 {
                continue;
            }
            return true;
        }
    }
    false
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        d: usize,
        xy: [(usize, usize); n],
    }
    let mut freq = vec![vec![0; d]; d];
    for i in 0..n {
        let (mut x, mut y) = xy[i];
        x %= d;
        y %= d;
        freq[x][y] += 1;
    }
    // Assuming usize is 64-bit.
    assert_eq!(std::mem::size_of::<usize>(), 8);
    let mut hi: usize = d * n;
    let mut lo: usize = 0;
    while hi - lo > 1 {
        let mid = (hi + lo) / 2;
        if calc(d, &freq, mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    puts!("{}\n", hi - 1);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
