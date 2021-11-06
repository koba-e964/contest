#[allow(unused_imports)]
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
const MOD: i64 = 1_000_000_007;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

// Verified by: https://atcoder.jp/contests/abc160/submissions/26509495
trait LeaveOne<App = ()>: Default + Clone {
    type T: Default + Clone;
    type App;
    fn build(vals: &[Self::T], app: &Self::App) -> Self;
    fn leave_one(&self, excl: Self::T) -> Self::T;
    fn exchange_one(&self, excl: Self::T, incl: Self::T) -> Self::T;
    fn add_one(&self, incl: Self::T) -> Self::T;
    fn as_is(&self) -> Self::T;
}

struct Reroot<LOO: LeaveOne> {
    #[allow(unused)]
    pub dp1: Vec<LOO::T>,
    #[allow(unused)]
    pub dp2: Vec<Vec<LOO::T>>,
    #[allow(unused)]
    pub dp_loo: Vec<LOO>,
}

impl<LOO: LeaveOne> Reroot<LOO> {
    pub fn new(g: &[Vec<usize>], app: &LOO::App) -> Self {
        let n = g.len();
        let mut dp1 = vec![LOO::T::default(); n];
        let mut dp2 = vec![vec![]; n];
        let mut dp_loo = vec![LOO::default(); n];
        Self::dfs1(0, n, &g, &mut dp_loo, &mut dp2, app);
        Self::dfs2(0, n, &g, &mut dp1, &dp_loo, &mut dp2, &app, LOO::T::default());
        Reroot {
            dp1: dp1,
            dp2: dp2,
            dp_loo: dp_loo,
        }
    }
    fn dfs1(
        v: usize, par: usize, g: &[Vec<usize>],
        dp_loo: &mut [LOO], dp2: &mut [Vec<LOO::T>], app: &LOO::App,
    ) {
        let mut mydp2 = vec![LOO::T::default(); g[v].len()];
        let mut chval = vec![];
        for i in 0..g[v].len() {
            let w = g[v][i];
            if w == par { continue; }
            Self::dfs1(w, v, g, dp_loo, dp2, app);
            mydp2[i] = dp_loo[w].as_is();
            chval.push(mydp2[i].clone());
        }
        dp_loo[v] = LOO::build(&chval, app);
        dp2[v] = mydp2;
    }
    fn dfs2(
        v: usize, par: usize, g: &[Vec<usize>],
        dp1: &mut [LOO::T],
        dp_loo: &[LOO],
        dp2: &mut [Vec<LOO::T>],
        app: &LOO::App,
        passed: LOO::T,
    ) {
        for i in 0..g[v].len() {
            let w = g[v][i];
            if w == par {
                dp2[v][i] = passed.clone();
                continue;
            }
            let inherited = if par >= g.len() {
                dp_loo[v].leave_one(dp2[v][i].clone())
            } else {
                dp_loo[v].exchange_one(dp2[v][i].clone(), passed.clone())
            };
            Self::dfs2(w, v, g, dp1, dp_loo, dp2, app, inherited);
        }
        dp1[v] = if par >= g.len() {
            dp_loo[v].as_is()
        } else {
            dp_loo[v].add_one(passed)
        };
    }
}

#[derive(Clone, Default)]
struct E2 {
    e: MInt,
    var: MInt,
}

impl LeaveOne for E2 {
    type T = (MInt, MInt);
    type App = ();
    fn build(vals: &[Self::T], &(): &Self::App) -> Self {
        let inv2 = MInt::new(2).inv();
        let inv4 = inv2 * inv2;
        let mut e = MInt::new(1);
        let mut var = MInt::new(0);
        // mu^2/4 + sigma^2/2
        for &(b, d) in vals {
            e += b * inv2;
            var += d * inv2 + b * b * inv4;
        }
        E2 {
            e: e,
            var: var,
        }
    }
    fn leave_one(&self, (b, d): Self::T) -> Self::T {
        let inv2 = MInt::new(2).inv();
        let inv4 = inv2 * inv2;
        (self.e - b * inv2, self.var - d * inv2 - b * b * inv4)
    }
    fn exchange_one(&self, (b1, d1): Self::T, (b2, d2): Self::T) -> Self::T {
        let inv2 = MInt::new(2).inv();
        let inv4 = inv2 * inv2;
        (self.e + inv2 * (b2 - b1), self.var + inv2 * (d2 - d1) + inv4 * (b2 * b2 - b1 * b1))
    }
    fn add_one(&self, (b, d): Self::T) -> Self::T {
        let inv2 = MInt::new(2).inv();
        let inv4 = inv2 * inv2;
        (self.e + b * inv2, self.var + d * inv2 + b * b * inv4)
    }
    fn as_is(&self) -> Self::T {
        (self.e, self.var)
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn solve() {
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let reroot = Reroot::<E2>::new(&g, &());
    let mut tot = MInt::new(0);
    for i in 0..n {
        let (e, var) = reroot.dp1[i];
        tot += e * e + var;
    }
    tot *= MInt::new(2).pow(n as i64 - 1);
    println!("{}", tot);
}
