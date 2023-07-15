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
    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(x: i64) -> Self { Self::new(x) }
    }
} // mod mod_int

macro_rules! define_mod {
    ($struct_name: ident, $modulo: expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct $struct_name {}
        impl mod_int::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

// Depends on MInt.rs
fn fact_init(w: usize) -> (Vec<MInt>, Vec<MInt>) {
    let mut fac = vec![MInt::new(1); w];
    let mut invfac = vec![0.into(); w];
    for i in 1..w {
        fac[i] = fac[i - 1] * i as i64;
    }
    invfac[w - 1] = fac[w - 1].inv();
    for i in (0..w - 1).rev() {
        invfac[i] = invfac[i + 1] * (i as i64 + 1);
    }
    (fac, invfac)
}

fn perm_comp(a: &[usize], b: &[usize]) -> Vec<usize> {
    let n = a.len();
    assert_eq!(b.len(), n);
    let mut c = vec![0; n];
    for i in 0..n {
        c[i] = a[b[i]];
    }
    c
}

fn perm_inv(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut c = vec![0; n];
    for i in 0..n {
        c[a[i]] = i;
    }
    c
}

// Returns the least index of elements that are modified, wrapped with Some.
// If the entire array is reversed, it returns None instead.
// v's elements must be pairwise distinct.
fn next_permutation<T: Ord>(v: &mut [T]) -> Option<usize> {
    let mut tail_dec: usize = 1;
    let n = v.len();
    while tail_dec < n {
        if v[n - tail_dec - 1] > v[n - tail_dec] {
            tail_dec += 1;
        } else {
            break;
        }
    }
    // v[n - tail_dec .. n] is strictly decreasing
    if tail_dec < n {
        let x = n - tail_dec - 1;
        let mut y = n;
        {
            let pivot = &v[x];
            for i in (n - tail_dec..n).rev() {
                if v[i] > *pivot {
                    y = i;
                    break;
                }
            }
            assert!(y < n);
        }
        v.swap(x, y);
    }
    v[n - tail_dec..].reverse();
    if tail_dec < n {
        Some(n - tail_dec - 1)
    } else {
        None
    }
}

// https://yukicoder.me/problems/no/2384 (5)
// F は S_N の自己同型群である。
// (i) N = 2 のとき、F の要素は恒等写像のみ。これが条件を満たすので答えは 1 である。
// (ii) N != 2, 6 のとき、これは S_N と同型であり、要素は共役作用のみである。
// 条件を満たす F の要素と 1 から K までをなんらかの円環シフトする置換は 1:1 に対応するので、答えは (N-K)!K である。
// (iii) N = 6 のとき、the outer automorphism が存在する。
// それは単一の automorphism に共役作用を合成したものなので、共役作用を全列挙すれば良い。
fn main() {
    let n: usize = get();
    let k: usize = get();
    if n == 2 {
        println!("1");
        return;
    }
    let (fac, _invfac) = fact_init(n + 1);
    if n != 6 {
        println!("{}", fac[n - k] * k as i64);
        return;
    }
    let mut tot = fac[n - k] * k as i64;
    // One representative of the outer automorphism.
    // https://mathstoshare.com/2019/12/16/the-outer-automorphism-of-s6/
    let outer = vec![
        vec![1, 0, 3, 2, 5, 4],
        vec![2, 4, 0, 5, 1, 3],
        vec![4, 5, 3, 2, 0, 1],
        vec![2, 3, 0, 1, 5, 4],
        vec![5, 4, 3, 2, 1, 0],
    ];
    let mut f: Vec<_> = (0..6).collect();
    for i in 0..k - 1 {
        f = perm_comp(&outer[i], &f);
    }
    let mut p: Vec<_> = (0..6).collect();
    loop {
        let pinv = perm_inv(&p);
        let q = perm_comp(&pinv, &perm_comp(&f, &p));
        if (0..k).all(|i| q[i] == (i + 1) % k) {
            tot += 1;
        }
        if let None = next_permutation(&mut p) {
            break;
        }
    }
    println!("{}", tot);
}
