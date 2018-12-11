#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        p: i64,
        q: usize,
        lr: [(i64, i64); q],
    }
    let mut tbl = vec![(0i64, 0i64, 0i64, 0i64)];
    let k = (p as f64).sqrt() as i64;
    for i in 1 .. p / k + 1 {
        let last = tbl[tbl.len() - 1].1;
        tbl.push((i, last + (p % i), 0, 0)); // 0, 0: don't care
    }
    for i in (1 .. k).rev() {
        let l = p / (i + 1);
        let r = p / i;
        let last = tbl[tbl.len() - 1].1;
        // delta = \sum_{j=l+1}^{r} (p % j) = \sum_{j=i+1}^{r} (p - i * j)
        let delta = (2 * p - i * (l + r + 1)) * (r - l) / 2;
        tbl.push((r, last + delta, i, p - i * r));
    }
    // eprintln!("tbl = {:?}", tbl);
    macro_rules! sum {
        ($k: expr) => {{
            let mut ans = 0;
            let mut k = $k;
            if k > p {
                ans += p * (k - p);
                k = p;
            }
            let idx = match tbl.binary_search(&(k, -1, -1, -1)) {
                Ok(_) => panic!(),
                Err(idx) => idx,
            };
            let (start, acc, d, b) = tbl[idx];
            let diff = start - k;
            ans + acc - b * diff - d * (diff * (diff - 1) / 2)
        }};
    }
    for (l, r) in lr {
        puts!("{}\n", sum!(r) - sum!(l - 1));
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
