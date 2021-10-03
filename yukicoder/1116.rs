use std::cmp::*;
use std::collections::*;
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

fn dfs(v: usize, par: usize,
       hm: &HashMap<usize, Vec<usize>>, vis: &mut HashSet<usize>) -> usize {
    if vis.contains(&v) {
        return 0;
    }
    vis.insert(v);
    let mut s = 1;
    for &w in &hm[&v] {
        if w == par { continue; }
        s += dfs(w, v, hm, vis);
    }
    s
}

// Tags: inclusion-exclusion, combinatorics
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize1, usize1); m],
    }
    let (fac, invfac) = fact_init(n + m + 1);
    let mut acc = vec![vec![vec![MInt::new(0); n + 2]; m + 1]; 2 * m + 1];
    // acc[x][y][z] = \sum_{0 <= i < z} C(n - x, i) * (i + y - 1)! * 2^{y - 1}
    for x in 0..min(n, 2 * m) + 1 {
        let mut cur = MInt::new(2).inv();
        for y in 0..m + 1 {
            for i in if y == 0 { 1 } else { 0 }..n + 1 {
                let tmp = if i <= n - x {
                    fac[n - x] * invfac[n - x - i] * invfac[i] * fac[i + y - 1]
                } else {
                    0.into()
                };
                acc[x][y][i + 1] = acc[x][y][i] + tmp * cur;
            }
            cur *= 2;
        }
    }
    let mut tot = MInt::new(0);
    'outer:
    for bits in 0usize..1 << m {
        let odd = bits.count_ones() % 2 == 1;
        let mut hm = HashMap::new();
        for i in 0..m {
            if (bits & 1 << i) != 0 {
                let (a, b) = ab[i];
                hm.entry(a).or_insert(vec![]).push(b);
                hm.entry(b).or_insert(vec![]).push(a);
            }
        }
        let mut f = [0; 3];
        for (&_, v) in &hm {
            if v.len() >= 3 {
                continue 'outer;
            }
            f[v.len()] += 1;
        }
        let mut vis = HashSet::new();
        let mut np = 0;
        let mut np1 = 0;
        for (&k, v) in &hm {
            if v.len() == 1 {
                let c = dfs(k, n, &hm, &mut vis);
                if c > 0 {
                    np += 1;
                    if c == 2 {
                        np1 += 1;
                    }
                }
            }
        }
        let mut nc = 0;
        for (&k, _) in &hm {
            if !vis.contains(&k) {
                nc += 1;
                dfs(k, n, &hm, &mut vis);
            }
        }
        // The graph is a sum of paths and cycles.
        if np > 0 && nc > 0 {
            continue;
        }
        if nc > 0 {
            if nc >= 2 {
                continue;
            }
            if odd {
                tot -= 1;
            } else {
                tot += 1;
            }
            continue;
        }
        let nr = n - hm.len();
        let mut tmp = MInt::new(0);
        // Find \sum_{0 <= i <= nr, 0 <= j <= np1, 0 <= k <= np - np1}
        // C(nr, i) (i+j+k-1)! * 2^{j+k-1} [i + 2j + 3k >= 3]
        // where j = np1, k = np - np1
        {
            let j = np1;
            let k = np - np1;
            let imin = max(2 * j + 3 * k, 3) - (2 * j + 3 * k);
            let tbl = &acc[hm.len()][j + k];
            if imin <= nr {
                tmp += (tbl[nr + 1] - tbl[imin])
                    * fac[np1] * invfac[j] * invfac[np1 - j]
                    * fac[np - np1] * invfac[k] * invfac[np - np1 - k];
            }
        }
        if odd {
            tot -= tmp;
        } else {
            tot += tmp;
        }
    }
    println!("{}", tot);
}
