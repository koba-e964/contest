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

fn convolution(a: &[MInt], b: &[MInt]) -> Vec<MInt> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let n = a.len() - 1;
    let m = b.len() - 1;
    let mut ans = vec![MInt::new(0); n + m + 1];
    for i in 0..n + 1 {
        for j in 0..m + 1 {
            ans[i + j] += a[i] * b[j];
        }
    }
    ans
}

// Finds [x^n] p(x)/q(x)
// Ref: https://qiita.com/ryuhe1/items/da5acbcce4ac1911f47a
// Verified by: https://atcoder.jp/contests/tdpc/submissions/24583334
// Depends on: MInt.rs
fn bostan_mori(p: &[MInt], q: &[MInt], mut n: i64) -> MInt {
    if p.is_empty() {
        return 0.into();
    }
    assert!(p.len() < q.len());
    let mut p = p.to_vec();
    let mut q = q.to_vec();
    while n > 0 {
        let mut qn = q.clone();
        for i in 0..qn.len() {
            if i % 2 == 1 {
                qn[i] = -qn[i];
            }
        }
        let num = convolution(&p, &qn);
        let den = convolution(&q, &qn);
        let mut nxt_p = vec![MInt::new(0); q.len() - 1];
        let mut nxt_q = vec![MInt::new(0); q.len()];
        for i in 0..q.len() - 1 {
            let to = 2 * i + (n % 2) as usize;
            if to < num.len() {
                nxt_p[i] = num[to];
            }
        }
        for i in 0..q.len() {
            nxt_q[i] = den[2 * i];
        }
        p = nxt_p;
        q = nxt_q;
        n /= 2;
    }
    p[0] * q[0].inv()
}

// https://yukicoder.me/problems/no/3572 (3)
// Solved with hints
// 根の集合は、あるdに対して1のd乗根全体、あるいは0、あるいはそれらのunionでなければならない。
// 元々の問題の答えを a(n) とし、0を考えないときの答えを b(n) とすると、 a(n) = \sum_{1<=k<=n}b(k) + 1 が成立する。
// b(n) は d ごとに分けると理解しやすい。b(n) のうち、根の集合が1のd乗根全体であるものを c(d,n) と呼ぶ。
// x^d-1 のQ上の因数分解の次数の多重集合を f(d) とすると、 c(d,n) は c(d,d) = 1 から開始して、 f(d) でナップサック数え上げをして得られる数列である。
// -> ChatGPT に聞いたら問題文を誤読していることがわかった。係数は実数なので、f(d) としてもR上の因数分解の次数を見るべきである。
// こうなると f(d) の要素は 1,2 のみであり、 b(n) の母関数は簡単に計算でき (x+x^2)/(1-x-2x^2+2x^3) である。
// これにより a(n) の母関数も計算でき、 (1-x^2+2x^3)/(1-2x-x^2+4x^3-2x^4) である。
fn main() {
    let n = getline().trim().parse::<i64>().unwrap();
    let p = vec![MInt::new(1), MInt::new(0), -MInt::new(1), MInt::new(2)];
    let q = vec![MInt::new(1), -MInt::new(2), -MInt::new(1), MInt::new(4), -MInt::new(2)];
    for k in 1..10 {
        eprintln!("{k} => {}", bostan_mori(&p, &q, k));
    }
    println!("{}", bostan_mori(&p, &q, n));
}
