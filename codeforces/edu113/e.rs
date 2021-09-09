use std::collections::*;
use std::io::{Write, BufWriter};
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

fn dfs(l: i64, r: i64, rnk: i64, a: MInt) -> Vec<(MInt, i64, Vec<i64>)> {
    let nxtrnk = if rnk <= 2 { 3 } else { 2 * rnk - 1 };
    if r - l == 1 {
        let t = vec![if rnk <= 2 { rnk } else { 0 }];
        return vec![(if rnk <= 2 { a.pow(t[0]) * l } else { 0.into() }, l, t)];
    }
    let mid = (r + l) / 2;
    let fst = dfs(l, mid, nxtrnk, a);
    let snd = dfs(mid, r, nxtrnk, a);
    let mut ans = vec![];
    let now = a.pow(nxtrnk);
    let fin = if rnk <= 2 { a.pow(rnk) } else { MInt::new(0) };
    for (v1, n1, t1) in fst {
        for &(v2, n2, ref t2) in &snd {
            let tmp = v1 + v2;
            let mut t = t1.clone();
            t.extend(t2);
            t[(n2 - l) as usize] = nxtrnk;
            if rnk <= 2 {
                t[(n1 - l) as usize] = rnk;
            }
            ans.push((tmp + now * n2 + fin * n1, n1, t.clone()));
            t[(n2 - l) as usize] = 0;
            t[(n1 - l) as usize] = nxtrnk;
            if rnk <= 2 {
                t[(n2 - l) as usize] = rnk;
            }
            ans.push((tmp + now * n1 + fin * n2, n2, t));
        }
    }
    ans
}

fn calc(a: &[(MInt, i64, Vec<i64>)], b: &[(MInt, i64, Vec<i64>)], h: MInt)
        -> Option<Vec<i64>> {
    let mut hm = HashMap::new();
    for i in 0..a.len() {
        hm.insert(a[i].0, i);
    }
    for &(b, _, ref t) in b {
        let res = h - b;
        if let Some(&idx) = hm.get(&res) {
            let mut t1 = a[idx].2.clone();
            t1.extend(t);
            return Some(t1);
        }
    }
    None
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input!(k: usize, a: i64, h: i64);
    let fst0 = dfs(1, (1 << (k - 1)) + 1, 1, a.into());
    let fst1 = dfs(1, (1 << (k - 1)) + 1, 2, a.into());
    let snd0 = dfs((1 << (k - 1)) + 1, (1 << k) + 1, 1, a.into());
    let snd1 = dfs((1 << (k - 1)) + 1, (1 << k) + 1, 2, a.into());
    let h = MInt::new(h);
    if let Some(v) = calc(&fst0, &snd1, h) {
        putvec!(v);
        return;
    }
    if let Some(v) = calc(&fst1, &snd0, h) {
        putvec!(v);
        return;
    }
    puts!("-1\n");
}
