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
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

/// Verified by https://atcoder.jp/contests/arc093/submissions/3968098
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
type ModInt = mod_int::ModInt<P>;

// Verified by: yukicoder No.1112
// https://yukicoder.me/submissions/510746
// https://en.wikipedia.org/wiki/Berlekamp%E2%80%93Massey_algorithm
// Complexity: O(n^2)
// Depends on ModInt.rs
fn berlekamp_massey<P: mod_int::Mod + PartialEq>(
    n: usize,
    s: &[mod_int::ModInt<P>],
) -> Vec<mod_int::ModInt<P>>{
    type ModInt<P> = mod_int::ModInt<P>;
    let mut b = ModInt::new(1);
    let mut cp = vec![ModInt::new(0); n + 1];
    let mut bp = vec![mod_int::ModInt::new(0); n];
    cp[0] = mod_int::ModInt::new(1);
    bp[0] = mod_int::ModInt::new(1);
    let mut m = 1;
    let mut l = 0;
    for i in 0..2 * n + 1 {
        assert!(i >= l);
        assert!(l <= n);
        if i == 2 * n { break; }
        let mut d = s[i];
        for j in 1..l + 1 {
            d += cp[j] * s[i - j];
        }
        if d == ModInt::new(0) {
            m += 1;
            continue;
        }
        if 2 * l > i {
            // cp -= d/b * x^m * bp
            let factor = d * b.inv();
            for j in 0..n + 1 - m {
                cp[m + j] -= factor * bp[j];
            }
            m += 1;
            continue;
        }
        let factor = d * b.inv();
        let tp = cp.clone();
        for j in 0..n + 1 - m {
            cp[m + j] -= factor * bp[j];
        }
        bp = tp;
        b = d;
        l = i + 1 - l;
        m = 1;
    }
    cp[0..l + 1].to_vec()
}

fn polymul(a: &[ModInt], b: &[ModInt], mo: &[ModInt]) -> Vec<ModInt> {
    let n = a.len();
    debug_assert_eq!(b.len(), n);
    debug_assert_eq!(mo.len(), n + 1);
    debug_assert_eq!(mo[n], 1.into());
    let mut ret = vec![ModInt::new(0); 2 * n - 1];
    for i in 0..n {
        for j in 0..n {
            ret[i + j] += a[i] * b[j];
        }
    }
    for i in (n..2 * n - 1).rev() {
        let val = ret[i];
        for j in 0..n {
            ret[i - n + j] -= val * mo[j];
        }
    }
    ret[..n].to_vec()
}
    

fn polypow(a: &[ModInt], mut e: i64, mo: &[ModInt]) -> Vec<ModInt> {
    let n = a.len();
    debug_assert_eq!(mo.len(), n + 1);
    let mut prod = vec![ModInt::new(0); n];
    prod[0] += 1;
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            prod = polymul(&prod, &cur, mo);
        }
        cur = polymul(&cur, &cur, mo);
        e /= 2;
    }
    prod
}

// Finds u a^e v^T by using Berlekamp-massey algorithm.
fn eval_matpow(a: &[Vec<ModInt>], e: i64, u: &[ModInt], v: &[ModInt]) -> ModInt {
    let k = a.len();
    // Find first 2k terms
    let mut terms = vec![ModInt::new(0); 2 * k];
    let mut cur = u.to_vec();
    for pos in 0..2 * k {
        for i in 0..k {
            terms[pos] += cur[i] * v[i];
        }
        let mut nxt = vec![ModInt::new(0); k];
        for i in 0..k {
            for j in 0..k {
                nxt[j] += cur[i] * a[i][j];
            }
        }
        cur = nxt;
    }
    let mut poly = berlekamp_massey(k, &terms);
    poly.reverse();
    // If terms' minimal polynomial is 1, then terms == 0.
    if poly.len() == 1 {
        return ModInt::new(0);
    }
    // If terms' minimal polynomial is x + r,
    // then terms is a gemetric progression with common ratio -r.
    // terms[i] = a * (-r)^i.
    if poly.len() == 2 {
        let r = -poly[0];
        return terms[0] * r.pow(e);
    }
    let mut base = vec![ModInt::new(0); poly.len() - 1];
    base[1] += 1;
    let powpoly = polypow(&base, e, &poly);
    let mut ans = ModInt::new(0);
    for i in 0..poly.len() - 1 {
        ans += powpoly[i] * terms[i];
    }
    ans
}

// Depends on ModInt.rs
fn fact_init(w: usize) -> (Vec<ModInt>, Vec<ModInt>) {
    let mut fac = vec![ModInt::new(1); w];
    let mut invfac = vec![0.into(); w];
    for i in 1 .. w {
        fac[i] = fac[i - 1] * i as i64;
    }
    invfac[w - 1] = fac[w - 1].inv();
    for i in (0 .. w - 1).rev() {
        invfac[i] = invfac[i + 1] * (i as i64 + 1);
    }
    (fac, invfac)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: [i64],
    }
    let (fac, invfac) = fact_init(100);
    const W: usize = 80;
    let mut sols = vec![ModInt::new(0); W];
    for i in 5..W {
        let mut tot = ModInt::new(0);
        for k in 0..i / 2 + 1 {
            if i - 2 * k < 5 { continue; }
            let tmp = fac[k + 4] * invfac[k] * invfac[4];
            let tmp = tmp * fac[i - 2 * k + 5] * invfac[10] * invfac[i - 2 * k - 5];
            tot += tmp;
        }
        sols[i] = tot;
    }
    let mut poly = berlekamp_massey(W / 2, &sols);
    poly.reverse();
    let mut base = vec![ModInt::new(0); poly.len() - 1];
    base[1] += 1;
    for n in n {
        let powpoly = polypow(&base, n, &poly);
        let mut ans = ModInt::new(0);
        for i in 0..poly.len() - 1 {
            ans += powpoly[i] * sols[i];
        }
        puts!("{}\n", ans);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
