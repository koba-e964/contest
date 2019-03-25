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
        xy: [(i64, i64); n],
    }
    const INF: i64 = 1 << 40;
    let (mut mix, mut miy) = ((INF, 0), (INF, 0));
    let (mut maax, mut maay) = ((-INF, 0), (-INF, 0));
    for i in 0..n {
        let (x, y) = xy[i];
        mix = min(mix, (x, i));
        miy = min(miy, (y, i));
        maax = max(maax, (x, i));
        maay = max(maay, (y, i));
    }
    let mut seqx: Vec<(i64, usize)> = (0..n).map(|i| (xy[i].0, i)).collect();
    let mut seqy: Vec<(i64, usize)> = (0..n).map(|i| (xy[i].1, i)).collect();
    seqx.sort();
    seqy.sort();
    let four = 2 * (maax.0 - mix.0 + maay.0 - miy.0);
    let mut three = 0;
    let find = |a: usize, b: usize| {
        let mut ma = 0;
        for i in 0..n {
            if i == a || i == b { continue; }
            let mut x = [0; 3];
            let mut y = [0; 3];
            x[0] = xy[a].0;
            y[0] = xy[a].1;
            x[1] = xy[b].0;
            y[1] = xy[b].1;
            x[2] = xy[i].0;
            y[2] = xy[i].1;
            x.sort();
            y.sort();
            ma = max(ma, y[2] - y[0] + x[2] - x[0]);
        }
        2 * ma
    };
    let mut rel = vec![];
    for i in 0..2 {
        rel.push(seqx[i].1);
        rel.push(seqx[n - 1 - i].1);
        rel.push(seqy[i].1);
        rel.push(seqy[n - 1 - i].1);
    }
    for i in 0..rel.len() {
        let a = rel[i];
        for j in 0..rel.len() {
            let b = rel[j];
            if a < b {
                three = max(three, find(a, b));
            }
        }
    }
    puts!("{}\n", three);
    for _ in 4..n + 1 {
        puts!("{}\n", four);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
