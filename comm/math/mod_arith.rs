const MOD: i64 = 1_000_000_007;

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

fn invmod(x: i64, m: i64) -> i64 {
    let (g, mut y, _) = ext_gcd(x, m);
    assert_eq!(g, 1);
    y %= m;
    if y < 0 {
        y += m;
    }
    y
}

// Find a generator of (Z/pZ)^\times
fn gen_zpz(p: i64) -> i64 {
    let mut v = p - 1;
    let mut f = 2;
    let mut fs = vec![];
    while v >= f * f {
        if v % f == 0 {
            fs.push(f);
            while v % f == 0 { v /= f; }
        }
        f += 1;
    }
    if v > 1 {
        fs.push(v);
    }
    let mut g = 2;
    loop {
        if fs.iter().all(|&x| powmod(g, (p - 1) / x, p) != 1) {
            return g;
        }
        g += 1;
    }
}

// Verified by: https://atcoder.jp/contests/ttpc2019/submissions/23338287
// Verified by: https://atcoder.jp/contests/acl1/submissions/23895636
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
    let rem0 = (q0 as i128 * y as i128) % m as i128;
    let rem1 = (q1 as i128 * x as i128) % n as i128;
    let val = rem0 as i64 * n + rem1 as i64 * m;
    let ret = val * g + (a % g);
    assert_eq!(ret % m, a % m);
    assert_eq!(ret % n, b % n);
    ret % (m / g * n)
}

const THRESHOLD: i64 = 60;

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

/**
 * Calculates x s.t. x^2 = a (mod p)
 * p is prime
 * Verified by: CF #395 Div1-C
 *              (http://codeforces.com/contest/763/submission/24380573)
 */
fn modsqrt(mut a: i64, p: i64) -> Option<i64> {
    a %= p;
    if a == 0 {
        return Some(0);
    }
    if p == 2 {
        return Some(a);
    }
    if powmod(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    let mut b = 1;
    while powmod(b, (p - 1) / 2, p) == 1 {
        b += 1;
    }
    let mut e = 0;
    let mut m = p - 1;
    while m % 2 == 0 {
        m /= 2;
        e += 1;
    }
    let mut x = powmod(a, (m - 1) / 2, p);
    let mut y = a * (x * x % p) % p;
    x = x * a % p;
    let mut z = powmod(b, m, p);
    while y != 1 {
        let mut j = 0;
        let mut t = y;
        while t != 1 {
            j += 1;
            t = t * t % p;
        }
        assert!(j < e);
        z = powmod(z, 1 << (e - j - 1), p);
        x = x * z % p;
        z = z * z % p;
        y = y * z % p;
        e = j;
    }
    Some(x)
}

#[derive(PartialEq,Eq,Hash,Clone,Copy,Debug)]
struct Hash {
    h: [i64; 2],
}
const MD: [i64; 2] = [1_000_000_007, 1_000_000_009];

impl Hash {
    fn new() -> Self { Hash::from(0) }
    fn from(v: i64) -> Self {
        Hash { h: [(v % MD[0] + MD[0]) % MD[0],
                   (v % MD[1] + MD[1]) % MD[1]] }
    }
}
impl std::ops::Add for Hash {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut ret = Self::new();
        for i in 0 .. 2 {
            ret.h[i] = (self.h[i] + other.h[i]) % MD[i];
        }
        ret
    }
}
impl std::ops::Neg for Hash {
    type Output = Self;
    fn neg(self) -> Self {
        let mut ret = Self::new();
        for i in 0 .. 2 {
            ret.h[i] = (MD[i] - self.h[i]) % MD[i];
        }
        ret
    }
}
impl std::ops::Mul for Hash {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut ret = Self::new();
        for i in 0 .. 2 {
            ret.h[i] = (self.h[i] * other.h[i]) % MD[i];
        }
        ret
    }
}

/// Refers external ::MOD.
mod mod_int {
    use ::MOD;
    use std::ops::*;
    #[derive(Copy, Clone, Debug)]
    pub struct ModInt {
        pub x: i64,
    }
    impl ModInt {
        pub fn new(x: i64) -> Self { ModInt { x: x } }
        pub fn mul_fast(self, others: Self) -> Self {
            Self::new(self.x * others.x % MOD)
        }
    }
    impl Add for ModInt {
        type Output = Self;
        fn add(self, others: Self) -> Self {
            let mut sum = self.x + others.x;
            if sum >= MOD {
                sum -= MOD;
            }
            Self::new(sum)
        }
    }
    impl Sub for ModInt {
        type Output = Self;
        fn sub(self, others: Self) -> Self {
            let mut sum = self.x - others.x;
            if sum < 0 {
                sum += MOD;
            }
            Self::new(sum)
        }
    }
    impl Mul for ModInt {
        type Output = Self;
        fn mul(self, others: Self) -> Self {
            // Naive multiplication in order to avoid overflow
            let mut sum = 0;
            let mut cur = self.x;
            let mut e = others.x;
            if self.x < others.x {
                cur = others.x;
                e = self.x;
            }
            while e > 0 {
                if e % 2 == 1 {
                    sum += cur;
                    if sum >= MOD {
                        sum -= MOD;
                    }
                }
                cur *= 2;
                if cur >= MOD {
                    cur -= MOD;
                }
                e /= 2;
            }
            Self::new(sum)
        }
    }
    impl BitXor<i64> for ModInt {
        type Output = Self;
        fn bitxor(self, others: i64) -> Self {
            Self::new(::powmod(self.x, others, MOD))
        }
    }
} // mod mod_int
