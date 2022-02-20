use std::cmp::*;
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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

/// Verified by https://atcoder.jp/contests/abc198/submissions/21774342
mod mod_int {
    use std::ops::*;
    pub trait Mod: Copy { fn m() -> i64; }
    #[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ModInt<M> { pub x: i64, phantom: ::std::marker::PhantomData<M> }
    impl<M: Mod> ModInt<M> {
        // x >= 0
        pub fn new(x: i64) -> Self { ModInt::new_internal(x % M::m()) }
        fn new_internal(x: i64) -> Self {
            ModInt { x: x, phantom: ::std::marker::PhantomData }
        }
        pub fn pow(self, mut e: i64) -> Self {
            debug_assert!(e >= 0);
            let mut sum = ModInt::new_internal(1);
            let mut cur = self;
            while e > 0 {
                if e % 2 != 0 { sum *= cur; }
                cur *= cur;
                e /= 2;
            }
            sum
        }
        #[allow(dead_code)]
        pub fn inv(self) -> Self { self.pow(M::m() - 2) }
    }
    impl<M: Mod> Default for ModInt<M> {
        fn default() -> Self { Self::new_internal(0) }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Add<T> for ModInt<M> {
        type Output = Self;
        fn add(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x + other.x;
            if sum >= M::m() { sum -= M::m(); }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Sub<T> for ModInt<M> {
        type Output = Self;
        fn sub(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x - other.x;
            if sum < 0 { sum += M::m(); }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Mul<T> for ModInt<M> {
        type Output = Self;
        fn mul(self, other: T) -> Self { ModInt::new(self.x * other.into().x % M::m()) }
    }
    impl<M: Mod, T: Into<ModInt<M>>> AddAssign<T> for ModInt<M> {
        fn add_assign(&mut self, other: T) { *self = *self + other; }
    }
    impl<M: Mod, T: Into<ModInt<M>>> SubAssign<T> for ModInt<M> {
        fn sub_assign(&mut self, other: T) { *self = *self - other; }
    }
    impl<M: Mod, T: Into<ModInt<M>>> MulAssign<T> for ModInt<M> {
        fn mul_assign(&mut self, other: T) { *self = *self * other; }
    }
    impl<M: Mod> Neg for ModInt<M> {
        type Output = Self;
        fn neg(self) -> Self { ModInt::new(0) - self }
    }
    impl<M> ::std::fmt::Display for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
    impl<M: Mod> ::std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let (mut a, mut b, _) = red(self.x, M::m());
            if b < 0 {
                a = -a;
                b = -b;
            }
            write!(f, "{}/{}", a, b)
        }
    }
    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(x: i64) -> Self { Self::new(x) }
    }
    // Finds the simplest fraction x/y congruent to r mod p.
    // The return value (x, y, z) satisfies x = y * r + z * p.
    fn red(r: i64, p: i64) -> (i64, i64, i64) {
        if r.abs() <= 10000 {
            return (r, 1, 0);
        }
        let mut nxt_r = p % r;
        let mut q = p / r;
        if 2 * nxt_r >= r {
            nxt_r -= r;
            q += 1;
        }
        if 2 * nxt_r <= -r {
            nxt_r += r;
            q -= 1;
        }
        let (x, z, y) = red(nxt_r, r);
        (x, y - q * z, z)
    }
} // mod mod_int

macro_rules! define_mod {
    ($struct_name: ident, $modulo: expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct $struct_name {}
        impl mod_int::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
const MOD: i64 = 1_000_000_007;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

type K = i64;
type V = MInt;
struct QuoDP {
    // stores dp[n], dp[n/2], ..., dp[n/b].
    dp_big: Vec<V>,
    dp: Vec<V>,
    n: K,
    b: K,
}

impl QuoDP {
    pub fn new(n: K, b: K) -> Self {
        let dp_big = vec![0.into(); b as usize + 1];
        let dp = vec![0.into(); (n / b) as usize];
        QuoDP {
            dp_big: dp_big,
            dp: dp,
            n: n,
            b: b,
        }
    }
    #[allow(unused)]
    pub fn keys(&self) -> Vec<K> {
        (1..self.n / self.b).chain((1..=self.b).rev().map(|x| self.n / x)).collect()
    }
    // pos should be of form floor(n / ???).
    fn upd<F>(&mut self, pos: K, f: F) where F: Fn(V) -> V {
        if pos >= self.n / self.b {
            let idx = self.n / pos;
            debug_assert_eq!(pos, self.n / idx);
            self.dp_big[idx as usize] = f(self.dp_big[idx as usize]);
            return;
        }
        let idx = pos as usize;
        self.dp[idx] = f(self.dp[idx]);
    }
    fn get(&self, pos: K) -> V {
        if pos >= self.n / self.b {
            let idx = self.n / pos;
            debug_assert_eq!(pos, self.n / idx);
            return self.dp_big[idx as usize];
        }
        let idx = pos as usize;
        self.dp[idx]
    }
    #[allow(unused)]
    fn init<F>(&mut self, f: F) where F: Fn(K) -> V {
        self.upd_all(|k, _| f(k));
    }
    fn upd_all<F>(&mut self, f: F) where F: Fn(K, V) -> V {
        for i in 0..self.dp.len() {
            self.dp[i] = f(i as K, self.dp[i]);
        }
        for i in (1..self.dp_big.len()).rev() {
            self.dp_big[i] = f(self.n / i as K, self.dp_big[i]);
        }
    }
}

impl std::fmt::Debug for QuoDP {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.dp.len() {
            writeln!(f, "{}: {}", i, self.dp[i])?;
        }
        for i in (1..self.dp_big.len()).rev() {
            writeln!(f, "{}: {}", self.n / i as K, self.dp_big[i])?;
        }
        Ok(())
    }
}

// Tags: variant-of-lucys-sieve
// Complexity: O(m^{3/4})
fn main() {
    let n: i64 = get();
    let m: i64 = get();
    let mut sqm = 0;
    while sqm * sqm <= m {
        sqm += 1;
    }
    sqm -= 1;
    let mut dp = QuoDP::new(m, sqm);
    let fac = MInt::new(n as i64 - 1).inv();
    let keys: Vec<i64> = dp.keys();
    for pos in keys {
        let mut x = 1;
        while x * x <= pos {
            x += 1;
        }
        x -= 1;
        let mut tmp = MInt::new(n);
        for j in 2..=min(x, n) {
            tmp += dp.get(pos / j);
        }
        for j in max(1, pos / n)..pos / x {
            let l = pos / (j + 1);
            let r = min(n, pos / j);
            if l < r {
                tmp += dp.get(j) * (r - l);
            }
        }
        dp.upd(pos, |_| tmp * fac);
    }
    println!("{}", dp.get(m));
}
