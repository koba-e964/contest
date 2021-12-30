use std::cmp::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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

// https://yukicoder.me/problems/no/1510 (5)
// 被積分関数は、次数の合計が 2N であるように極を持つ。x = A_i sqrt(-1) における留数は、被積分関数内部の \prod における 1/(x^2 + A_i^2) の個数を a、それ以外の要素に対する 1/(-A_i^2 + A_j^2) の積を b とおくと、[x^{-1}]b/(x^2 + 2A_i sqrt(-1)x)^a = [x^{a-1}]b/(x + 2A_i sqrt(-1))^a = b/(2A_i sqrt(-1))^a [x^{a-1}](1+x/(2A_i sqrt(-1)))^{-a} = (-1)^{a+1} b * C(2a-1, a-1) / (2A_i sqrt(-1)) である。
// -R -> R -> (半径 R の上半平面上の半円) -> -R という経路による積分を考えると、これの積分値への寄与は 2sqrt(-1) (-1)^{a+1} b * C(2a-1, a-1) / (2A_i sqrt(-1)) = (-1)^{a+1} b C(2a-1, a-1) / A_i である。
// -> 留数を間違えて WA。留数の分母は 2A_i sqrt(-1) ではなく (2A_i sqrt(-1))^{2a-1} である。これの積分値への寄与は 2sqrt(-1) (-1)^{a+1} b * C(2a-1, a-1) / (2A_i sqrt(-1))^{2a-1} = 2b * C(2a-1, a-1) / (2A_i)^{2a-1} である。
// -> [Y^{a-1}](1-Y)^{-a} の計算も間違えていて、正しくは C(2a-1,a-1) ではなく C(2a-2, a-1) である。正しい留数は (-1)^{a+1} b * C(2a-2, a-1) / (2A_i sqrt(-1))^{2a-1} であり、正しい寄与は 2b * C(2a-2, a-1) / (2A_i)^{2a-1} である。
// -> x = A_i に対して a >= 2 のとき、単に 1/(-A_i^2 + A_j^2) の積を考えるだけではダメで、\prod_j 1/((x-A_i sqrt(-1))^2 + A_j^2) の x^{a-1} 次の項まで考える必要がある。同様に (1+x/(2A_i sqrt(-1)))^{-a} の方も x^{a-1} 次の項まで考える必要がある。欲しいものは 2 sqrt(-1) (2A_i sqrt(-1))^{-a} [x^{a-1}](1+x/(2A_i sqrt(-1)))^{-a} \prod_j 1/((x-A_i sqrt(-1))^2 + A_j^2) であり、畳み込みに a^2 時間かける場合計算量は O(\sum n a^2) = O(n^3) である。
// -> \prod の中身は 1/((x+A_i sqrt(-1))^2 + A_j^2) である。x = A_i における極の様子を調べるためには -A_i だけ平行移動してx = 0 での値を調べる必要があるため。
// Tags: complex-analysis, integral, residue-of-meromorphic-functions
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let im = MInt::new(3).pow((MOD - 1) / 4);
    let mut coo = a.clone();
    coo.sort(); coo.dedup();
    let m = coo.len();
    let mut f = vec![0; m];
    for &a in &a {
        f[coo.binary_search(&a).unwrap()] += 1;
    }
    let (fac, invfac) = fact_init(2 * n);
    let mut tot = MInt::new(0);
    for i in 0..m {
        let a = f[i];
        // \prod 1/((x+coo[i]*im)^2 + coo[j])^f[j]
        let mut frm = vec![MInt::new(0); a];
        frm[0] = 1.into();
        for j in 0..m {
            if i == j { continue; }
            let cs = MInt::new(coo[j]).pow(2) - coo[i] * coo[i];
            let csinv = cs.inv();
            for _ in 0..f[j] {
                let mut ep = vec![MInt::new(0); a];
                ep[0] = frm[0] * csinv;
                if a > 1 {
                    ep[1] = (-im * 2 * coo[i] * ep[0] + frm[1]) * csinv;
                }
                for k in 2..a {
                    ep[k] = (-im * 2 * coo[i] * ep[k - 1] + frm[k] - ep[k - 2]) * csinv;
                }
                frm = ep;
            }
        }
        // (1+x/(2*coo[i]*im))^a
        let mut lat = vec![MInt::new(0); a];
        let mut cur = MInt::new(1);
        let inv = (im * 2 * coo[i]).inv();
        for i in 0..a {
            lat[i] = fac[i + a - 1] * invfac[i] * invfac[a - 1] * cur;
            cur *= -inv;
        }
        let mut tmp = MInt::new(0);
        for j in 0..a {
            tmp += frm[a - 1 - j] * lat[j];
        }
        let tmp = tmp * inv.pow(a as i64) * 2 * im;
        tot += tmp;
    }
    println!("{}", tot);
}
