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

// Verified by: yukicoder No.1112
// https://yukicoder.me/submissions/510746
// https://en.wikipedia.org/wiki/Berlekamp%E2%80%93Massey_algorithm
// Complexity: O(n^2)
// Depends on MInt.rs
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

// Finds u a^e v^T by using Berlekamp-massey algorithm.
// The linear map a is given as a closure.
// Complexity: O(n^2 log e + nT(n)) where n = |u| and T(n) = complexity of a.
// Ref: https://yukicoder.me/wiki/black_box_linear_algebra
// Verified by: yukicoder No. 215 (https://yukicoder.me/submissions/854179)
// Depends on: fps/bostan_mori.rs
fn eval_matpow<F: FnMut(&[MInt]) -> Vec<MInt>>(mut a: F, e: i64, u: &[MInt], v: &[MInt]) -> MInt {
    let k = u.len();
    // Find first 2k terms
    let mut terms = vec![MInt::new(0); 2 * k];
    let mut cur = u.to_vec();
    for pos in 0..2 * k {
        for i in 0..k {
            terms[pos] += cur[i] * v[i];
        }
        cur = a(&cur);
    }
    let poly = berlekamp_massey(k, &terms);
    let mut nom = convolution(&terms[..poly.len() - 1], &poly);
    nom.truncate(poly.len() - 1);
    bostan_mori(&nom, &poly, e)
}

// #{x | x < m, (x as bitstring)[p] = 1}
fn count_pop_bits(m: i64, p: usize) -> i64 {
    let lead = m & ((-1) << (p + 1));
    let rest = m - lead;
    let ans = (lead >> 1) + if rest >= 1 << p { rest - (1 << p) } else { 0 };
    ans
}

// \sum_{x < m} (x xor 2^p) - x
pub fn c(m: i64, p: usize) -> MInt {
    let tmp = MInt::new(m) - count_pop_bits(m, p) * 2;
    tmp * MInt::new(2).pow(p as i64)
}

// \sum{x | x < m, (x as bitstring)[p] = 1}
pub fn e(m: i64, p: usize) -> MInt {
    let lead = m & ((-1) << (p + 1));
    let rest = m - lead;
    let p2 = MInt::new(1 << p);
    let inv2 = MInt::new(2).inv();
    let count = MInt::new(lead >> (p + 1));
    let mut tot = p2 * (p2 * 3 - 1) * inv2 * count;
    tot += count * (count - 1) * inv2 * MInt::new(1 << (p + 1)) * (1 << p);
    if rest >= 1 << p {
        let tmp = MInt::new(rest - (1 << p));
        tot += MInt::new(lead + (1 << p)) * tmp;
        tot += tmp * (tmp - 1) * inv2;
    }
    tot
}

// \sum_{k < m, (k as bitstring)[p] = 1} (2^i xor k) - k
pub fn d(m: i64, p: usize, i: usize) -> MInt {
    if p == i {
        return -MInt::new(count_pop_bits(m, p)) * (1 << i);
    }
    let lead = m & ((-1) << (p + 1));
    let rest = m - lead;
    if i < p {
        return if rest >= 1 << p {
            c(rest - (1 << p), i)
        } else {
            0.into()
        };
    }
    let mut ans = c(lead >> (p + 1), i - p - 1) * (1 << p) * (1 << (p + 1));
    let tmp = if rest >= 1 << p {
        MInt::new(rest - (1 << p)) * (1 << i)
    } else {
        MInt::new(0)
    };
    if (lead & (1 << i)) != 0 {
        ans -= tmp;
    } else {
        ans += tmp;
    }
    ans
}

// dp[i][j] := i 番目まで埋めて A_i= j のときの積の総和 とすると、dp[i] |-> dp[i+1] は線型変換。
// これを行列累乗する必要があるが、そのままだと次元が M であり大きすぎるのである程度まとめる必要がある。
// u_j = dp[i][j], v_j = dp[i+1][j] として、u から v を作る線型変換のより小さい不変部分空間を作る。
// v_j = \sum_k u_k (k xor j) である。
// a := (u_j の和), s_i := {i 番目ビットが立っているもの限定の u_j の和},
// b := (v_j の和), t_i := {i 番目ビットが立っているもの限定の v_j の和} とする。
// a, s_i から b, t_i が計算できるのがポイント。そのためには以下の補題を使う。
// 補題: k = 2^a + 2^b + ... とする。このとき (k xor j) - j = ((2^a xor j) - j) + ((2^b xor j) - j) + ...
// この補題を使うと、まず b = (M(M-1)/2) a + \sum c(2^i) s_i が言える。(c(x) := \sum_{j<M} ((x xor j) - j))
// 同様に t_j := \sum_{k<M, k の j ビット目は立っている} ka + \sum_i d(2^i) s_i
// (d_j(x) := \sum_{k<M, k の j ビット目は立っている} ((x xor k) - k)) が言える。
// (証明の方針: b - (M(M-1)/2) a = \sum_{j,k} u_k ((k xor j) - j) = \sum c(2^i) \sum_{k<n, k の i ビット目は立っている} u_k = \sum c(2^i) s_i)
// c(2^i), d_j(2^i), e_j := \sum_{k<M, k の j ビット目は立っている} k は高速に計算できる。
fn main() {
    let n: i64 = get();
    let m: i64 = get();
    let mut k = 0;
    while (1 << k) < m {
        k += 1;
    }
    let mut init = vec![MInt::new(0); k + 1];
    init[0] += m;
    for i in 0..k {
        init[i + 1] += count_pop_bits(m, i);
    }
    let mut pred = vec![MInt::new(0); k + 1];
    pred[0] += 1;
    let mut mat = vec![vec![MInt::new(0); k + 1]; k + 1];
    let mm = m % MOD;
    mat[0][0] += mm * (mm - 1) / 2;
    for i in 0..k {
        mat[i + 1][0] = c(m, i);
    }
    for j in 0..k {
        mat[0][j + 1] = e(m, j);
        for i in 0..k {
            mat[i + 1][j + 1] = d(m, j, i);
        }
    }
    let ans = eval_matpow(|v| {
        let mut res = vec![MInt::new(0); k + 1];
        for i in 0..k + 1 {
            for j in 0..k + 1 {
                res[j] += v[i] * mat[i][j];
            }
        }
        res
    }, n - 1, &init, &pred);
    println!("{}", ans);
}
