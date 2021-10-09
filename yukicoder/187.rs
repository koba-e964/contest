use std::collections::*;
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

// https://judge.yosupo.jp/submission/5155
mod pollard_rho {
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
            if (y & 1) == 1 { sum = add_mod(sum, cur, n); }
            cur = add_mod(cur, cur, n);
            y >>= 1;
        }
        sum
    }

    fn mod_pow(x: i64, mut e: i64, n: i64) -> i64 {
        let mut prod = if n == 1 { 0 } else { 1 };
        let mut cur = x % n;
        while e > 0 {
            if (e & 1) == 1 { prod = mul_mod(prod, cur, n); }
            e >>= 1;
            if e > 0 { cur = mul_mod(cur, cur, n); }
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
            for _ in 0..e {
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
                    if d != 1 { break; }
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
        if x <= 1 { return vec![]; }
        let mut hm = std::collections::HashMap::new();
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

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let r = a % b;
    let q = a / b;
    let (g, x, y) = ext_gcd(b, r);
    (g, y, x - q * y)
}

fn inv_mod(a: i64, b: i64) -> i64 {
    let (_, mut x, _) = ext_gcd(a, b);
    x %= b;
    if x < 0 {
        x += b;
    }
    x
}

// gcd(rm[i].1, rm[j].1) == 1 for i != j
// Ref: https://www.creativ.xyz/ect-gcd-crt-garner-927/
// O(n^2)
fn garner(rm: &[(i64, i64)], mo: i64) -> i64 {
    let n = rm.len();
    let mut x_mo = (rm[0].0 % rm[0].1) % mo;
    let mut mp_mo = 1;
    let mut coef = Vec::with_capacity(n);
    coef.push(rm[0].0 % rm[0].1);
    for i in 1..n {
        let (r, m) = rm[i];
        let r = r % m;
        let mut mp_mi = 1;
        let mut x_mi = 0;
        mp_mo = mp_mo * (rm[i - 1].1 % mo) % mo;
        for j in 0..i {
            x_mi = (x_mi + mp_mi * (coef[j] % m)) % m;
            mp_mi = mp_mi * (rm[j].1 % m) % m;
        }
        let t = (r - x_mi + m) % m * inv_mod(mp_mi, m) % m;
        x_mo = (x_mo + t % mo * mp_mo) % mo;
        coef.push(t);
    }
    x_mo
}

// Tags: chinese-remainder-theorem, garners-algorithm
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut hm = HashMap::new();
    for &(x, y) in &xy {
        let pe = pollard_rho::factorize(y);
        for &(p, e) in &pe {
            let mut v = 1;
            for _ in 0..e {
                v *= p;
            }
            hm.entry(p).or_insert(vec![]).push((v, x % v));
        }
    }
    let mut dat = vec![];
    for (_, mut v) in hm {
        v.sort();
        let (y, x) = v[v.len() - 1];
        for &(b, a) in &v {
            if a != x % b {
                println!("-1");
                return;
            }
        }
        dat.push((x, y))
    }
    const MOD: i64 = 1_000_000_007;
    let mut val = garner(&dat, MOD);
    // We need the positive maximum; if the result is 0, we need \prod m.
    if dat.iter().all(|&(r, _)| r == 0) {
        let mut prod = 1;
        for &(_, m) in &dat {
            prod = prod * m % MOD;
        }
        val = (val + prod) % MOD;
    }
    println!("{}", val);
}
