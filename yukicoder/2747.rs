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

// https://ferin-tech.hatenablog.com/entry/2019/08/11/%E3%83%A9%E3%82%B0%E3%83%A9%E3%83%B3%E3%82%B8%E3%83%A5%E8%A3%9C%E9%96%93
// Finds f(t) given y[i] = f(x0 + d * i) for 0 <= i < y.len().
// O(y.len() * log MOD)-time
fn lagrange_interpolate_one_arithprog(y: &[MInt], x0: MInt, d: MInt, t: MInt) -> MInt {
    assert_ne!(d, 0.into());
    let n = y.len();
    let mut sum = MInt::new(0);
    // (x-x0-d*i)/((x-x0)...(x-x0-d*(n-1)))|_{x=x0+d*i}
    let mut cur = MInt::new(1);
    // (t-x0)...(t-x0-d*(n-1))
    let mut tprod = MInt::new(1);
    for i in 1..n {
        cur *= -d * i as i64;
    }
    cur = cur.inv();
    for i in 0..n {
        if t == x0 + d * i as i64 {
            return y[i];
        }
        tprod *= t - x0 - d * i as i64;
    }
    for i in 0..n {
        sum += y[i] * cur * tprod * (t - x0 - d * i as i64).inv();
        if i + 1 < n {
            cur *= (n - i - 1) as i64;
            cur *= -MInt::new((i + 1) as i64).inv();
        }
    }
    sum
}

// Generated by 2747-help.rs
const STEP: usize = 10000000;
const LEN: usize = 100;
const FACT_TABLE: [i64; 100] = [
    1,
    295201906,
    160030060,
    957629942,
    545208507,
    213689172,
    760025067,
    939830261,
    506268060,
    39806322,
    808258749,
    440133909,
    686156489,
    741797144,
    390377694,
    12629586,
    544711799,
    104121967,
    495867250,
    421290700,
    117153405,
    57084755,
    202713771,
    675932866,
    79781699,
    956276337,
    652678397,
    35212756,
    655645460,
    468129309,
    761699708,
    533047427,
    287671032,
    206068022,
    50865043,
    144980423,
    111276893,
    259415897,
    444094191,
    593907889,
    573994984,
    892454686,
    566073550,
    128761001,
    888483202,
    251718753,
    548033568,
    428105027,
    742756734,
    546182474,
    62402409,
    102052166,
    826426395,
    159186619,
    926316039,
    176055335,
    51568171,
    414163604,
    604947226,
    681666415,
    511621808,
    924112080,
    265769800,
    955559118,
    763148293,
    472709375,
    19536133,
    860830935,
    290471030,
    851685235,
    242726978,
    169855231,
    612759169,
    599797734,
    961628039,
    953297493,
    62806842,
    37844313,
    909741023,
    689361523,
    887890124,
    380694152,
    669317759,
    367270918,
    806951470,
    843736533,
    377403437,
    945260111,
    786127243,
    80918046,
    875880304,
    364983542,
    623250998,
    598764068,
    804930040,
    24257676,
    214821357,
    791011898,
    954947696,
    183092975,
];

// https://yukicoder.me/problems/no/2747 (3.5)
// solved with hints
// \sum_{1 <= i <= N} (N-i)i^K が計算できれば良い。これはベルヌーイ数の先頭 K 項が O(K log K)-time 程度で計算できれば計算できる。
// -> 解説を見た。ラグランジュ補間の方が簡単。最終的な多項式は K+2 次なので、0 <= i <= K+2 の K+3 点で補間する。
// 最後に (N-2)! * (N-1) * 2 を掛けること。
// - (N-2)!: 残りの点の埋め方
// - (N-1): どの隙間を見るか
// - 2: 左の方が大きいか
// Tags: lagrange-polynomial-interpolation, lagrange-interpolation
fn main() {
    let n: i64 = get();
    let k: i64 = get();
    let mut y = vec![];
    let mut sum = MInt::new(0);
    for i in 0..k + 3 {
        sum += MInt::new(i).pow(k) * (n - i);
        y.push(sum);
    }
    let mut ans = lagrange_interpolate_one_arithprog(&y, 0.into(), 1.into(), n.into());
    ans *= 2;
    let tbl_idx = ((n - 1) as usize / STEP).min(LEN - 1);
    let mut fac = MInt::new(FACT_TABLE[tbl_idx]);
    for i in tbl_idx * STEP + 1..=(n - 1) as usize {
        fac *= i as i64;
    }
    ans *= fac;
    println!("{ans}");
}
