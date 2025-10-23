fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

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
        pub struct $struct_name {}
        impl mod_int::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

// Tags: complicated-states
// 9 変数からなる巨大な状態を使って処理する。
fn main() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut ans = MInt::new(0);
    let mut added2 = MInt::new(0);
    let mut term = MInt::new(1);
    let mut term2 = MInt::new(0);
    let mut cur = MInt::new(0);
    let mut cur2 = MInt::new(0);
    let mut dig = MInt::new(0);
    let mut cnt = MInt::new(0);
    let mut cnt2 = MInt::new(0);
    for &c in &s {
        if c == '*' || c == '+' {
            term2 = term2 * cur + cur2;
            term *= cur;
            cur = 0.into();
            cur2 = 0.into();
            dig = 0.into();
        }
        if c == '+' {
            added2 = added2 + term2 + term * cnt2;
            term = 1.into();
            term2 = 0.into();
            cnt2 = cnt;
        }
        if c == '*' || c == '+' {
            continue;
        }
        let c = (c as u8 - b'0') as i64;
        dig += 1;
        cur = cur * 10 + c;
        cur2 = cur2 * 10 + dig * c;
        cnt += 1;
        ans += added2 + term2 * cur + term * cur * cnt2 + cur2;
    }
    println!("{ans}");
    if n <= 100 {
        let mut debug = MInt::new(0);
        for i in 0..n {
            if s[i] == '+' || s[i] == '*' {
                continue;
            }
            for j in i..n {
                if s[j] == '+' || s[j] == '*' {
                    continue;
                }
                let mut ans = MInt::new(0);
                let mut cur = MInt::new(0);
                let mut stack = MInt::new(1);
                for &c in &s[i..=j] {
                    if c == '*' {
                        stack *= cur;
                        cur = MInt::new(0);
                        continue;
                    }
                    if c == '+' {
                        ans += stack * cur;
                        stack = 1.into();
                        cur = 0.into();
                        continue;
                    }
                    let c = (c as u8 - b'0') as i64;
                    cur = cur * 10 + c;
                }
                ans += stack * cur;
                debug += ans;
            }
        }
        eprintln!("debug = {debug}");
    }
}
