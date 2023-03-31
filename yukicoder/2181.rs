use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn factorize(mut x: i64) -> Vec<(i64, usize)> {
    let mut p = 2;
    let mut ans = vec![];
    while p * p <= x {
        let mut e = 0;
        while x % p == 0 {
            x /= p;
            e += 1;
        }
        if e > 0 {
            ans.push((p, e));
        }
        p += 1;
    }
    if x > 1 {
        ans.push((x, 1));
    }
    ans
}

// Verified by: https://yukicoder.me/submissions/706484
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
    if rm.is_empty() {
        return 0;
    }
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

// https://web.archive.org/web/20170202003812/http://www.dms.umontreal.ca/~andrew/PDF/BinCoeff.pdf
pub struct PrimePowComb {
    p: i64,
    e: usize,
    pe: i64,
    fac: Vec<i64>,
    invfac: Vec<i64>,
}

impl PrimePowComb {
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
    // O(p^e)
    // p must be a prime
    pub fn new(p: i64, e: usize) -> Self {
        assert!(p <= 1 << 31);
        let mut pe = 1i64;
        for _ in 0..e {
            pe = pe.saturating_mul(p);
        }
        assert!(pe <= 1 << 31);
        let pp = p as usize;
        let peu = pe as usize;
        let mut fac = vec![0; peu];
        let mut invfac = vec![0; peu];
        fac[0] = 1;
        for i in 1..peu {
            if i % pp == 0 {
                fac[i] = fac[i - 1];
            } else {
                fac[i] = fac[i - 1] * i as i64 % pe;
            }
        }
        invfac[peu - 1] = Self::powmod(fac[peu - 1], pe / p * (p - 1) - 1, pe);
        for i in (0..peu - 1).rev() {
            if i % pp == pp - 1 {
                invfac[i] = invfac[i + 1];
            } else {
                invfac[i] = invfac[i + 1] * (i + 1) as i64 % pe;
            }
        }
        PrimePowComb {
            p: p,
            e: e,
            pe: pe,
            fac: fac,
            invfac: invfac,
        }
    }
    // (a!)_p mod p^e, \prod_{1 <= i <= a, not (p | i)} i
    // O(1)
    pub fn fac_pe(&self, a: i64) -> i64 {
        let pe = self.pe;
        assert!(a < pe);
        self.fac[a as usize]
    }
    // 1/(a!)_p mod p^e
    // O(1)
    pub fn invfac_pe(&self, a: i64) -> i64 {
        let pe = self.pe;
        assert!(a < pe);
        self.invfac[a as usize]
    }
    // Find ord_p(C(a, b)).
    // O(log_p a)-time
    pub fn comb_ord(&self, mut a: i64, mut b: i64, from: usize) -> i64 {
        if a < b { return -1; }
        let p = self.p;
        let mut c = a - b;
        let mut ans = 0;
        let mut carry = 0;
        let mut pos = 0;
        while a > 0 {
            if b % p + c % p + carry >= p {
                if pos >= from {
                    ans += 1;
                }
                carry = 1;
            } else {
                carry = 0;
            }
            a /= p;
            b /= p;
            c /= p;
            pos += 1;
        }
        ans
    }
    // Find C(a, b) mod p^e.
    // O(e * log_p a)-time
    pub fn comb(&self, mut a: i64, mut b: i64) -> i64 {
        if a < b {
            return 0;
        }
        let ord = self.comb_ord(a, b, 0);
        let p = self.p;
        let pe = self.pe;
        let sgn = self.comb_ord(a, b, self.e - 1);
        let mut c = a - b;
        let mut res = 1;
        while a > 0 {
            let aw = a % pe;
            let bw = b % pe;
            let cw = c % pe;
            res = res * self.fac_pe(aw) % pe
                * self.invfac_pe(bw) % pe
                * self.invfac_pe(cw) % pe;
            a /= p;
            b /= p;
            c /= p;
        }
        for _ in 0..ord {
            res = res * p % pe;
        }
        if p >= 3 || pe <= 4 {
            if sgn % 2 != 0 {
                res = (pe - res) % pe;
            }
        }
        res
    }
}

// https://yukicoder.me/problems/no/2181 (3.5)
// 問題の値は \sum_{L <= n <= R} (C(2n, n) - 2) である。
// C(2n, n) mod M は https://web.archive.org/web/20170202003812/http://www.dms.umontreal.ca/~andrew/PDF/BinCoeff.pdf で計算できる。
// Tags: combination-mod
fn main() {
    let l: i64 = get();
    let r: i64 = get();
    let m: i64 = get();
    let pe = factorize(m);
    let mut res = vec![];
    for &(p, e) in &pe {
        let comb = PrimePowComb::new(p, e);
        let pe = comb.pe;
        let mut ans = 0;
        for i in l..=r {
            ans = (ans + comb.comb(2 * i, i) + pe - 2) % pe;
        }
        res.push((ans, pe));
    }
    println!("{}", garner(&res, m));
}
