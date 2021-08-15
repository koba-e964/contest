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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
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
const MOD: i64 = 1_000_000_007;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

// [2, 4, 0, 1, 3, 7, 6] ==> [[0, 2], [1, 4, 3], [6, 7]]
// Verified by: https://atcoder.jp/contests/joisc2007/submissions/24248388
fn decompose_into_cycles(a: &[usize]) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut vis = vec![false; n];
    let mut ans = vec![];
    for i in 0..n {
        if vis[i] { continue; }
        vis[i] = true;
        let mut cyc = vec![i];
        let mut v = a[i];
        while v != i {
            vis[v] = true;
            cyc.push(v);
            v = a[v];
        }
        ans.push(cyc)
    }
    ans
}

// Depends on MInt.rs
fn fact_init(w: usize) -> (Vec<MInt>, Vec<MInt>) {
    let mut fac = vec![MInt::new(1); w];
    let mut invfac = vec![0.into(); w];
    for i in 1 .. w {
        fac[i] = fac[i - 1] * i as i64;
    }
    invfac[w - 1] = fac[w - 1].inv();
    for i in (0 .. w - 1).rev() {
        invfac[i] = invfac[i + 1] * (i as i64 + 1);
    }
    (fac, invfac)
}

fn main() {
    input! {
        n: usize,
        p: [usize1; n],
        q: [usize1; n],
    }
    let (fac, _invfac) = fact_init(n + 1);
    let mut a = vec![0; n];
    for i in 0..n {
        a[p[i]] = q[i];
    }
    let cyc = decompose_into_cycles(&a);
    let mut dp = vec![MInt::new(0); n + 1];
    let mut ep = vec![MInt::new(0); n + 1];
    dp[0] = 1.into();
    let mut sz = 1;
    let mut help = vec![vec![]; n + 1];
    let mut help2 = vec![vec![]; n + 1];
    let mut incl = vec![vec![]; n + 1];
    help[0] = vec![[MInt::new(1), MInt::new(0)]; 1];
    help2[0] = vec![[MInt::new(1), MInt::new(0)]; 1];
    incl[0] = vec![MInt::new(0); 1];
    for i in 1..n + 1 {
        help[i] = vec![[MInt::new(0); 2]; i + 1];
        help2[i] = vec![[MInt::new(0); 2]; i + 1];
        for j in 0..i + 1 {
            let mut me = [MInt::new(0); 2];
            let mut me2 = [MInt::new(0); 2];
            if j < i {
                me[0] += help[i - 1][j][0] + help[i - 1][j][1];
                me2[0] += help2[i - 1][j][0];
                me2[0] += help[i - 1][j][1];
            }
            if j > 0 {
                let x = help2[i - 1][j - 1][0] + help2[i - 1][j - 1][1];
                me[1] += help[i - 1][j - 1][0] + help[i - 1][j - 1][1] + x;
                me2[1] += x;
            }
            help[i][j] = me;
            help2[i][j] = me2;
        }
    }
    for i in 0..min(n + 1, 5) {
        eprintln!("help[{}] = {:?}", i, help[i]);
        eprintln!("help2[{}] = {:?}", i, help2[i]);
    }
    for cyc in cyc {
        for i in 0..n + 1 {
            ep[i] = 0.into();
        }
        let m = cyc.len();
        let mut tmp = vec![MInt::new(0); m + 1];
        for i in 0..m - 1 {
            for j in 1..i + 1 {
                tmp[i] += (help[m - j - 2][i - j][0] + help[m - j - 2][i - j][1]) * j as i64 * (j + 1) as i64;
            }
            tmp[i] += help[m - 1][i][0] + help[m - 1][i][1];
        }
        tmp[m - 1] += m as i64 * m as i64;
        tmp[m] += 2;
        if m == 1 {
            tmp[1] = 1.into();
        }
        for i in 0..m + 1 {
            for j in 0..sz {
                ep[i + j] += dp[j] * tmp[i];
            }
        }
        eprintln!("{} ==> {:?}", m, tmp);
        sz += m;
        std::mem::swap(&mut dp, &mut ep);
    }
    assert_eq!(sz, n + 1);
    let mut tot = MInt::new(0);
    eprintln!("dp = {:?}", dp);
    for i in 0..n + 1 {
        let tmp = dp[i] * fac[n - i];
        if i % 2 == 0 {
            tot += tmp;
        } else {
            tot -= tmp;
        }
    }
    println!("{}", tot);
}
