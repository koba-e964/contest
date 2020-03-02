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

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

fn extgcd(x: i64, y: i64) -> (i64, i64, i64) {
    if y == 0 {
        return (x, 1, 0);
    }
    let r = x % y;
    let q = x / y;
    let (g, a, b) = extgcd(y, r);
    (g, b, a - b * q)
}

// Baby-step giant-step. https://codeforces.com/contest/1106/submission/49502575
// Find x s.t. x^exp = m
fn modroot(m: i64, exp: i64, p: i64) -> Option<i64> {
    // prime factors of p - 1
    let mut rel = vec![];
    {
        let mut v = p - 1;
        let mut q = 2;
        while v > 1 && v >= q * q {
            if v % q == 0 {
                rel.push(q);
                while v % q == 0 {
                    v /= q;
                }
            }
            q += 1;
        }
        if v > 1 {
            rel.push(v);
        }
    }
    let mut gen = 2;
    loop {
        let ok = rel.iter().all(|q| powmod(gen, (p - 1) / q, p) != 1);
        if ok {
            break;
        }
        gen += 1;
    }
    const SQRT: i64 = 40000;
    let mut cur = 1;
    let prog = powmod(gen, SQRT, p);
    let mut tbl = HashMap::new();
    for i in 0..SQRT {
        tbl.insert(cur, SQRT * i);
        cur = cur * prog % p;
    }
    let discrete_log = |x: i64| {
        let mut cur = x;
        for i in 0..SQRT {
            let key = cur;
            if let Some(&y) = tbl.get(&key) {
                return (y - i + p - 1) % (p - 1);
            }
            cur = cur * gen % p;
        }
        unreachable!();
    };
    let logm = discrete_log(m);
    let (g, inv, _) = extgcd(exp, p - 1);
    if logm % g != 0 {
        return None;
    }
    let inv = (inv + p - 1) % (p - 1);
    let ans = (logm / g * inv) % (p - 1);
    Some(powmod(gen, ans, p))
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        pka: [(i64, i64, i64)],
    }
    for (p, k, a) in pka {
        assert!(2 <= p && p <= 1_000_000_007);
        let ans = modroot(a, k, p);
        puts!("{}\n", ans.unwrap_or(-1));
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
