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
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

/// Generates an Iterator over subsets of univ, in the descending order. 
/// Verified by: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3050308
struct SubsetIter { bits: Option<usize>, univ: usize }
impl Iterator for SubsetIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self.bits {
            None => None,
            Some(bits) => {
                let ans = bits;
                self.bits =
                    if bits == 0 { None }
                else { Some((bits - 1) & self.univ) };
                Some(ans)
            }
        }
    }
}
fn subsets(univ: usize) -> SubsetIter {
    SubsetIter { bits: Some(univ), univ: univ }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn calc(s: u32, g: &[u32], k: i64) -> MInt {
    assert!(s.count_ones() <= 15);
    let m = s.count_ones() as usize;
    let n = g.len();
    if s == 0 {
        return 1.into();
    }
    let mut map = vec![0; n];
    let mut pos = 0;
    for i in 0..n {
        if (s & 1 << i) != 0 {
            map[i] = pos;
            pos += 1;
        }
    }
    let mut indep = vec![false; 1 << m];
    let mut comp = vec![0u32; m];
    for i in 0..n {
        if (s & 1 << i) == 0 { continue; }
        for j in 0..n {
            if i == j || (s & 1 << j) == 0 || (g[i] & 1 << j) == 0 { continue; }
            comp[map[i]] |= 1 << map[j];
        }
    }
    for bits in 1..1 << m {
        let mut ok = true;
        for i in 0..m {
            if (bits & 1 << i) != 0 {
                ok &= (comp[i] & bits) == 0;
            }
        }
        indep[bits as usize] = ok;
    }
    let mut ans = MInt::new(0);
    let mut dp = vec![MInt::new(0); 1 << m];
    dp[0] += 1;
    let mut cur = MInt::new(1);
    for i in 0..m {
        let mut ep = vec![MInt::new(0); 1 << m];
        for bits in 0usize..1 << m {
            let lsb = bits & bits.wrapping_neg();
            for sub in subsets(bits - lsb) {
                let sub = sub + lsb;
                if indep[sub] {
                    ep[bits] += dp[bits - sub];
                }
            }
        }
        dp = ep;
        cur *= k - i as i64 + MOD;
        ans += dp[(1 << m) - 1] * cur;
    }
    ans
}

fn dfs(s: u32, g: &[u32], k: i64) -> MInt {
    if s == 0 {
        return 1.into();
    }
    let n = g.len();
    for i in 0..n {
        if (s & 1 << i) == 0 {
            continue;
        }
        if (g[i] & s) == 0 {
            return dfs(s ^ 1 << i, g, k) * k;
        }
        if (g[i] & s).count_ones() == 1 {
            return dfs(s ^ 1 << i, g, k) * (k - 1);
        }
    }
    if (0..n).all(|i| (s & 1 << i) == 0 || (g[i] & s).count_ones() >= 4) {
        return calc(s, &g, k);
    }
    // deletion - contraction
    let mut cont = g.to_vec();
    let mut del = g.to_vec();
    let mut idx = (4, n);
    for i in 0..n {
        if (s & 1 << i) != 0 && (g[i] & s).count_ones() <= 3 {
            idx = std::cmp::min(idx, ((g[i] & s).count_ones(), i));
        }
    }
    let idx = idx.1;
    let b = g[idx] & s;
    let b = b & b.wrapping_neg();
    let to = (b - 1).count_ones() as usize;
    assert_ne!(g[idx] & 1 << to, 0);
    assert_ne!(g[to] & 1 << idx, 0);
    del[idx] ^= 1 << to;
    del[to] ^= 1 << idx;
    let mut ans = dfs(s, &del, k);
    cont[idx] |= cont[to];
    cont[idx] ^= 1 << idx;
    for i in 0..n {
        if i != idx && (cont[i] & 1 << to) != 0 {
            cont[i] |= 1 << idx;
        }
    }
    for i in 0..n {
        assert_eq!(cont[i] & 1 << i, 0);
    }
    ans -= dfs(s ^ 1 << to, &cont, k);
    ans
}

// O^*(1.84^m)
// Tags: chromatic-polynomials, deletion-contraction, exponential-algorithms
fn solve() {
    input! {
        n: usize, m: usize, k: i64,
        uv: [(usize1, usize1); m],
    }
    let mut g = vec![0; n];
    for (u, v) in uv {
        g[u] |= 1 << v;
        g[v] |= 1 << u;
    }
    println!("{}", dfs((1 << n) - 1, &g, k));
}
