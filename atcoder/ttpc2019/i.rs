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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

/// https://judge.yosupo.jp/submission/5155
mod pollard_rho {
    use std::collections::HashMap;
    /// binary gcd
    pub fn gcd(mut x: i64, mut y: i64) -> i64 {
        if y == 0 { return x; }
        if x == 0 { return y; }
        let k = (x | y).trailing_zeros();
        y >>= k;
        x >>= x.trailing_zeros();
        while y != 0 {
            y >>= y.trailing_zeros();
            if x > y { let t = x; x = y; y = t; }
            y -= x;
        }
        x << k
    }

    fn add_mod(x: i64, y: i64, n: i64) -> i64 {
        let z = x + y;
        if z >= n { z - n } else { z }
    }

    fn mul_mod(x: i64, mut y: i64, n: i64) -> i64 {
        assert!(x >= 0);
        assert!(x < n);
        let mut sum = 0;
        let mut cur = x;
        while y > 0 {
            if (y & 1) == 1 {
                sum = add_mod(sum, cur, n);
            }
            cur = add_mod(cur, cur, n);
            y >>= 1;
        }
        sum
    }

    fn mod_pow(x: i64, mut e: i64, n: i64) -> i64 {
        let mut prod = if n == 1 { 0 } else { 1 };
        let mut cur = x % n;
        while e > 0 {
            if (e & 1) == 1 {
                prod = mul_mod(prod, cur, n);
            }
            e >>= 1;
            if e > 0 {
                cur = mul_mod(cur, cur, n);
            }
        }
        prod
    }

    pub fn is_prime(n: i64) -> bool {
        if n <= 1 { return false; }
        let small = [2, 3, 5, 7, 11, 13];
        if small.iter().any(|&u| u == n) { return true; }
        if small.iter().any(|&u| n % u == 0) { return false; }
        let mut d = n - 1;
        let e = d.trailing_zeros();
        d >>= e;
        // https://miller-rabin.appspot.com/
        let a = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
        a.iter().all(|&a| {
            if a % n == 0 { return true; }
            let mut x = mod_pow(a, d, n);
            if x == 1 { return true; }
            for _ in 0 .. e {
                if x == n - 1 {
                    return true;
                }
                x = mul_mod(x, x, n);
                if x == 1 { return false; }
            }
            x == 1
        })
    }

    fn pollard_rho(n: i64, c: &mut i64) -> i64 {
        // An improvement with Brent's cycle detection algorithm is performed.
        // https://maths-people.anu.edu.au/~brent/pub/pub051.html
        if n % 2 == 0 { return 2; }
        loop {
            let mut x: i64; // tortoise
            let mut y = 2; // hare
            let mut d = 1;
            let cc = *c;
            let f = |i| add_mod(mul_mod(i, i, n), cc, n);
            let mut r = 1;
            // We don't perform the gcd-once-in-a-while optimization
            // because the plain gcd-every-time algorithm appears to
            // outperform, at least on judge.yosupo.jp :)
            while d == 1 {
                x = y;
                for _ in 0..r {
                    y = f(y);
                    d = gcd((x - y).abs(), n);
                    if d != 1 {
                        break;
                    }
                }
                r *= 2;
            }
            if d == n {
                *c += 1;
                continue;
            }
            return d;
        }
    }

    /// Outputs (p, e) in p's ascending order.
    pub fn factorize(x: i64) -> Vec<(i64, usize)> {
        if x <= 1 {
            return Vec::new();
        }
        let mut hm = HashMap::new();
        let mut pool = vec![x];
        let mut c = 1;
        while let Some(u) = pool.pop() {
            if is_prime(u) {
                *hm.entry(u).or_insert(0) += 1;
                continue;
            }
            let p = pollard_rho(u, &mut c);
            pool.push(p);
            pool.push(u / p);
        }
        let mut v: Vec<_> = hm.into_iter().collect();
        v.sort();
        v
    }
} // mod pollard_rho

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

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let r = a % b;
    let q = a / b;
    let (g, x, y) = ext_gcd(b, r);
    (g, y, x - q * y)
}

fn invmod(x: i64, m: i64) -> i64 {
    let (g, mut y, _) = ext_gcd(x, m);
    assert_eq!(g, 1);
    y %= m;
    if y < 0 {
        y += m;
    }
    y
}

fn garner((a, m): (i64, i64), (b, n): (i64, i64)) -> i64 {
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
    let ret = val * g + (a % g);
    assert_eq!(ret % m, a % m);
    assert_eq!(ret % n, b % n);
    ret % (m / g * n)
}

fn calc(l: i64, r: i64, p: i64, a: i64, b: usize) -> i64 {
    let mut apow = 1;
    for _ in 0..b {
        apow *= a;
    }
    let n = apow as usize;
    let mut tbl = vec![0; n];
    tbl[0] = 1;
    for i in 1..n {
        let v = i as i64;
        let cur = if v % a == 0 {
            1
        } else {
            v
        };
        tbl[i] = tbl[i - 1] * cur % apow;
    }
    let fact = |mut x: i64| -> (i64, i64) {
        let mut prod = 1;
        let mut e = 0;
        let mut c = 0;
        while x > 0 {
            let q = x / apow;
            let r = x % apow;
            prod *= tbl[r as usize];
            prod %= apow;
            prod = prod * powmod(tbl[n - 1], q, apow) % apow;
            if c > 0 {
                e += x;
            }
            x /= a;
            c += 1;
        }
        (prod, e)
    };
    let (r0, r1) = fact(r);
    let (l0, l1) = fact(l - 1);
    let invl0 = invmod(l0, apow);
    let mut v0 = r0 * invl0 % apow;
    let mut v1 = r1 - l1;
    // p
    let mut p0 = p;
    let mut p1 = 0;
    while p0 % a == 0 {
        p0 /= a;
        p1 += 1;
    }
    let red = |mut x: i64| -> i64 {
        let mut s = 0;
        while x > 0 {
            x /= p;
            s += x;
        }
        s
    };
    let tmp = red(r) - red(l - 1);
    let p0inv = invmod(p0 % apow, apow);
    v0 = v0 * powmod(p0inv, tmp, apow) % apow;
    v1 = v1 - p1 * tmp;
    v0 * powmod(a, v1, apow) % apow
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        p: i64, q: i64,
        l: i64, r: i64,
    }
    let pe = pollard_rho::factorize(q);
    let mut val = 0;
    let mut lcm = 1;
    for &(a, b) in &pe {
        let sub = calc(l, r, p, a, b);
        let mut prod = 1;
        for _ in 0..b {
            prod *= a;
        }
        eprintln!("mod {} ==> {}", prod, sub);
        val = garner((val, lcm), (sub, prod));
        lcm *= prod;
    }
    puts!("{}\n", val);
}
