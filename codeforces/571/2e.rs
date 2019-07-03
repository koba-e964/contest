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
        m: usize,
        q: usize,
        a: [chars; n],
        qs: [(i64, i64, i64, i64); q],
    }
    let mut acc = vec![vec![0i32; 2 * m + 1]; 2 * n + 1];
    for i in 0..2 * n {
        for j in 0..2 * m {
            let flip = (i / n) ^ (j / m);
            acc[i + 1][j + 1] = acc[i][j + 1] + acc[i + 1][j] - acc[i][j] +
                (i32::from(a[i % n][j % m] == '1') ^ flip as i32);
        }
    }
    let nn = n as i64;
    let mm = m as i64;
    let range = |a: i64, b: i64| {
        let qa = a / (2 * nn);
        let qb = b / (2 * mm);
        let ra = a % (2 * nn);
        let rb = b % (2 * mm);
        let mut tot = a * qb * mm + rb * qa * nn;
        let mut tmp = acc[ra as usize][rb as usize] as i64;
        if (qa ^ qb).count_ones() % 2 == 1 {
            let whole = ra * rb;
            tmp = whole - tmp;
        }
        tot += tmp;
        tot
    };
    for &(x1, y1, x2, y2) in &qs {
        let x1 = x1 - 1;
        let y1 = y1 - 1;
        let mut tot = 0;
        tot += range(x2, y2);
        tot -= range(x1, y2);
        tot -= range(x2, y1);
        tot += range(x1, y1);
        puts!("{}\n", tot);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
