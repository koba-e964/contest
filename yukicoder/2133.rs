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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
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

fn divide_segments(mut lr: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut ans = vec![];
    while let Some((mut l, mut r)) = lr.pop() {
        let mut new = vec![];
        for &(x, y) in &ans {
            if y <= l || r <= x {
                new.push((x, y));
                continue;
            }
            if (l <= x && y <= r) || (x <= l && r <= y) {
                new.push((x, y));
                continue;
            }
            if x < l {
                lr.push((x, l));
                lr.push((l, y));
                lr.push((y, r));
                l = 0;
                r = 0;
                continue;
            }
            assert!(l < x);
            lr.push((l, x));
            lr.push((x, r));
            lr.push((r, y));
            l = 0;
            r = 0;
        }
        if l < r {
            new.push((l, r));
        }
        ans = new;
    }
    ans.sort_by_key(|&(l, r)| (r - l, l));
    ans.dedup();
    ans
}

// https://yukicoder.me/problems/no/2133 (3.5)
// 奇数を +1、偶数を -1 とし、+1, -1 の累積和に対する問題と考えるとわかりやすい。
// 二つの区間 [l1, r1), [l2, r2) が包含関係になく共通部分を持つ場合、
// 累積和の min はどちらも同じ値である。
// これを利用して区間を細分化することができる。これを繰り返すと最終的に、
// (2 つの区間が共通部分を持つ ==> それらは包含関係にある) を満たす区間の集合が得られる。
// その集合に対し小さい順 (長さの昇順でよい) に考えると解ける。
fn main() {
    input! {
        n: usize, q: usize,
        lr: [(usize1, usize); q],
    }
    let (fac, invfac) = fact_init(n + 1);
    let trans = divide_segments(lr);
    let mut ans = MInt::new(1);
    let mut rem = n;
    let mut seen = vec![false; n + 1];
    for &(l, r) in &trans {
        let mut count = 0;
        for i in l..r {
            if !seen[i] {
                seen[i] = true;
                count += 1;
                rem -= 1;
            }
        }
        if count % 2 == 1 {
            ans = 0.into();
        } else {
            ans *= fac[count] * invfac[count / 2] * invfac[count / 2 + 1];
        }
    }
    let paths = |x: usize| {
        if x <= rem && (x + rem) % 2 == 0 {
            fac[rem] * invfac[(rem - x) / 2] * invfac[(rem + x) / 2]
        } else {
            MInt::new(0)
        }
    };
    let ge = if rem % 2 == 0 {
        paths(0) - paths(2)
    } else {
        paths(1) - paths(3)
    };
    ans *= fac[n - n / 2] * fac[n / 2];
    println!("{}", ge * ans);
}
