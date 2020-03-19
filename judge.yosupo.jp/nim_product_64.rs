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

fn calc(a: u64, b: u64, width: u32) -> u64 {
    // Two principles:
    // (1) x < 2^{2^k} => prod(x, 2^{2^k}) = x * 2^{2^k}
    // (2) prod(2^{2^k}, 2^{2^k}) = 3 * 2^{2^k} / 2
    if width == 1 {
        return a & b;
    }
    let width = width / 2;
    let (ahi, alo) = (a >> width, a & ((1 << width) - 1));
    let (bhi, blo) = (b >> width, b & ((1 << width) - 1));
    let hi = calc(ahi, bhi, width);
    let lo = calc(alo, blo, width);
    let upper = calc(ahi ^ alo, bhi ^ blo, width) ^ lo;
    let lower = lo ^ calc(hi, 1 << (width - 1), width);
    upper << width | lower
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input!(ab: [(u64, u64)]);
    let mut precalc = [[0; 64]; 64];
    for i in 0..64 {
        for j in 0..64 {
            precalc[i][j] = calc(1 << i, 1 << j, 64);
        }
    }
    for (a, b) in ab {
        let mut sum = 0;
        for i in 0..64 {
            if (a & 1 << i) == 0 { continue; }
            for j in 0..64 {
                if (b & 1 << j) != 0 {
                    sum ^= precalc[i][j];
                }
            }
        }
        puts!("{}\n", sum);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
