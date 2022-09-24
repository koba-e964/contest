// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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

// Find min x >= 0 s.t. a^x = b
// Verified by: https://atcoder.jp/contests/abc270/submissions/35138605
// Depends on: math/mod_arith.rs
fn bsgs(p: i64, a: i64, b: i64) -> i64 {
    assert!(a >= 0 && a < p);
    assert!(b >= 0 && b < p);
    if a == 0 {
        return if b == 1 { 0 } else if b == 0 { 1 } else { -1 };
    }
    const B: i64 = 32000;
    let ap = powmod(a, B, p);
    let mut hm = std::collections::HashMap::new();
    let mut cur = 1;
    for i in 0..B {
        if !hm.contains_key(&cur) {
            hm.insert(cur, i * B);
        }
        cur *= ap;
        cur %= p;
    }
    let mut mi = p;
    let ainv = powmod(a, p - 2, p);
    cur = b;
    for i in 0..B {
        if let Some(val) = hm.get(&cur) {
            mi = std::cmp::min(mi, val + i);
        }
        cur *= ainv;
        cur %= p;
    }
    let res = if mi >= p {
        -1
    } else {
        mi
    };
     res
}

fn calc(p: i64, a: i64, b: i64, s: i64, g: i64) -> i64 {
    if a == 1 {
        if b == 0 {
            return if s == g { 0 } else { -1 };
        }
        let invb = powmod(b, p - 2, p);
        return invb * ((g + p - s) % p) % p;
    }
    let inva1 = powmod((a + p - 1) % p, p - 2, p);
    let c = inva1 * b;
    let s = (s + c) % p;
    let g = (g + c) % p;
    if s == 0 {
        return if g == 0 { 0 } else { -1 };
    }
    let quo = g * powmod(s, p - 2, p) % p;
    bsgs(p, a, quo)
}

fn main() {
    input! {
        t: usize,
        c: [(i64, i64, i64, i64, i64); t],
    }
    for (p, a, b, s, g) in c {
        println!("{}", calc(p, a, b, s, g));
    }
}
