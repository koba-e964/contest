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

const THRESHOLD: i64 = 60;

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let r = a % b;
    let q = a / b;
    let (g, x, y) = ext_gcd(b, r);
    (g, y, x - q * y)
}

fn garner_thresh((a, m): (i64, i64), (b, n): (i64, i64)) -> i64 {
    assert!(0 <= a);
    assert!(0 <= b);
    if a == b {
        return a;
    }
    let (g, mut x, mut y) = ext_gcd(m, n);
    assert_eq!(a % g, b % g);
    let m = m / g;
    let n = n / g;
    let q0 = a / g;
    let q1 = b / g;
    x %= n;
    if x < 0 {
        x += n;
    }
    y %= m;
    if y < 0 {
        y += m;
    }
    let val = (q0 * y) % m * n + (q1 * x) % n * m;
    let mut ret = val * g + (a % g);
    let l = m * n * g;
    let least = (THRESHOLD + l - 1) / l * l;
    if ret < least {
        ret = least + ret % l;
    }
    assert_eq!(ret % m, a % m);
    assert_eq!(ret % n, b % n);
    assert!(ret >= THRESHOLD);
    ret
}

fn phi(mut x: i64) -> i64 {
    let mut p = 2;
    let mut ret = 1;
    while p * p <= x {
        let mut e = 0;
        while x % p == 0 {
            x /= p;
            e += 1;
        }
        if e > 0 {
            ret *= p - 1;
            for _ in 1..e { ret *= p; }
        }
        p += 1;
    }
    if x > 1 {
        ret *= x - 1;
    }
    ret
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

fn powmod_thresh(x: i64, e: i64, mo: i64) -> i64 {
    let least = (THRESHOLD + mo - 1) / mo * mo;
    if x == 0 {
        return if e == 0 { 1 } else { 0 };
    }
    if x == 1 {
        return 1;
    }
    if e <= 7 {
        let mut v = 1;
        for _ in 0..e { v *= x; }
        return if v >= least { (v - least) % mo + least } else { v };
    }
    let v = powmod(x, e, mo);
    if v < THRESHOLD {
        v + least
    } else {
        v
    }
}

fn calc_tower(a: i64, m: i64) -> (i64, i64, i64) {
    assert!(a >= 2);
    let mut phiseq = vec![m];
    while let Some(&last) = phiseq.last() {
        if last == 1 { break; }
        let next = phi(last);
        phiseq.push(next);
    }
    // 2^(2^(2^2)) = 65536
    for _ in 0..4 {
        phiseq.push(1);
    }
    let returned_phi = phiseq[1];
    let mut val = 1;
    let mut vals = vec![];
    while let Some(phi) = phiseq.pop() {
        val = powmod_thresh(a, val, phi);
        vals.push(val);
    }
    let val0 = vals.pop().unwrap();
    let val1 = vals.pop().unwrap();
    (val0, val1, returned_phi)
}

fn calc(a: i64, m: i64) -> i64 {
    if a == 0 {
        return m;
    }
    if a == 1 {
        return 1;
    }
    let (val0, val1, phi) = calc_tower(a, m);
    let val = garner_thresh((val0, m), (val1, phi));
    assert_eq!(powmod(a, val, m), val % m);
    val
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input!(am: [(i64, i64)]);
    for &(a, m) in &am {
        puts!("{}\n", calc(a, m));
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
