#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

/// Verified by https://atcoder.jp/contests/arc093/submissions/3968098
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
const MOD: i64 = 998244353;
define_mod!(P, MOD);
type ModInt = mod_int::ModInt<P>;

fn squmul(a: &[Vec<ModInt>], b: &[Vec<ModInt>]) -> Vec<Vec<ModInt>> {
    let n = a.len();
    let mut ret = vec![vec![ModInt::new(0); n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ret[i][k] += a[i][j] * b[j][k];
            }
        }
    }
    ret
}

fn dotprod(a: &[Vec<ModInt>], v: &[ModInt]) -> Vec<ModInt> {
    let n = a.len();
    let mut ret = vec![ModInt::new(0); n];
    for i in 0..n {
        for j in 0..n {
            ret[i] += a[i][j] * v[j];
        }
    }
    ret
}

// Editorial's solution
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        a: [i64; n],
        m: usize,
        xyc: [(usize1, i64, usize1); m],
        trans: [[i32; 3]; 3],
    }
    let mut mat = vec![vec![ModInt::new(0); 64]; 64];
    let mut mat_ind = vec![vec![vec![ModInt::new(0); 64]; 64]; 3];
    for c in 0..3 {
        for g1 in 0..4 {
            for g2 in 0..4 {
                for g3 in 0..4 {
                    let mut reachable = vec![false; 4];
                    let g = [g1, g2, g3];
                    for i in 0..3 {
                        if trans[c][i] == 1 {
                            reachable[g[i]] = true;
                        }
                    }
                    let mut mex = 0;
                    while reachable[mex] {
                        mex += 1;
                    }
                    assert!(mex <= 3);
                    // g1 g2 g3 -> mex g1 g2
                    mat[16 * mex + 4 * g1 + g2][16 * g1 + 4 * g2 + g3] += 1;
                    mat_ind[c][16 * mex + 4 * g1 + g2][16 * g1 + 4 * g2 + g3] += 1;
                }
            }
        }
    }
    let mut pows = vec![];
    pows.push(mat);
    for i in 0..30 {
        let last = squmul(&pows[i], &pows[i]);
        pows.push(last);
    }
    let mut events = vec![vec![]; n];
    for &(x, y, c) in &xyc {
        events[x].push((y, c));
    }
    let mut dp = [ModInt::new(0); 4];
    dp[0] += 1;
    for i in 0..n {
        events[i].sort();
        let mut v = vec![ModInt::new(0); 64];
        v[63] += 1;
        let m = events[i].len();
        for j in 0..m + 1 {
            let dist = if j == m { a[i] } else { events[i][j].0 - 1 }
            - if j == 0 { 0 } else { events[i][j - 1].0 };
            for k in 0..30 {
                if (dist & 1 << k) != 0 {
                    v = dotprod(&pows[k], &v);
                }
            }
            if j < m {
                let c = events[i][j].1;
                v = dotprod(&mat_ind[c], &v);
            }
        }
        let mut res = vec![ModInt::new(0); 4];
        for i in 0..64 {
            res[i / 16] += v[i];
        }
        let mut ep = [ModInt::new(0); 4];
        for j in 0..4 {
            for k in 0..4 {
                ep[j ^ k] += dp[j] * res[k];
            }
        }
        dp = ep;
    }
    puts!("{}\n", dp[0]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
