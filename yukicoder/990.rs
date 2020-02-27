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

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y; y = r;
    }
    x
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, m: usize, k: i64,
        op: chars,
        b: [i64; m],
        a: [i64; n],
    }
    let mul = op == vec!['*'];
    if mul {
        let mut ha = HashMap::new();
        let mut hb = HashMap::new();
        for &a in &a {
            let g = gcd(a, k);
            *ha.entry(g).or_insert(0) += 1;
        }
        for &b in &b {
            let g = gcd(b, k);
            *hb.entry(g).or_insert(0) += 1;
        }
        let ha: Vec<_> = ha.into_iter().collect();
        let hb: Vec<_> = hb.into_iter().collect();
        let mut cnt: i64 = 0;
        for (d, ca) in ha {
            for &(e, cb) in &hb {
                if d * e % k == 0 {
                    cnt += ca * cb;
                }
            }
        }
        puts!("{}\n", cnt);
        return;
    }
    let mut ha = HashMap::new();
    let mut hb = HashMap::new();
    for &a in &a {
        let g = a % k;
        *ha.entry(g).or_insert(0) += 1;
    }
    for &b in &b {
        let g = (k - b % k) % k;
        *hb.entry(g).or_insert(0) += 1;
    }
    let mut cnt: i64 = 0;
    for (d, ca) in ha {
        if let Some(&cb) = hb.get(&d) {
            cnt += ca * cb;
        }
    }
    puts!("{}\n", cnt);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
