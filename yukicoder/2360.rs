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

// Verified by: https://yukicoder.me/submissions/717057
trait LeaveOne<Me = (), App = ()>: Default + Clone {
    type T: Default + Clone;
    type Me: Clone;
    type App;
    fn build(me: Self::Me, vals: &[Self::T], app: &Self::App) -> Self;
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
    pub fn new(g: &[Vec<usize>], me: &[LOO::Me], app: &LOO::App) -> Self {
        let n = g.len();
        let mut dp1 = vec![LOO::T::default(); n];
        let mut dp2 = vec![vec![]; n];
        let mut dp_loo = vec![LOO::default(); n];
        Self::dfs1(0, n, &g, &mut dp_loo, &mut dp2, me, app);
        Self::dfs2(0, n, &g, &mut dp1, &dp_loo, &mut dp2, &app, LOO::T::default());
        Reroot {
            dp1: dp1,
            dp2: dp2,
            dp_loo: dp_loo,
        }
    }
    fn dfs1(
        v: usize, par: usize, g: &[Vec<usize>],
        dp_loo: &mut [LOO], dp2: &mut [Vec<LOO::T>], me: &[LOO::Me], app: &LOO::App,
    ) {
        let mut mydp2 = vec![LOO::T::default(); g[v].len()];
        let mut chval = vec![];
        for i in 0..g[v].len() {
            let w = g[v][i];
            if w == par { continue; }
            Self::dfs1(w, v, g, dp_loo, dp2, me, app);
            mydp2[i] = dp_loo[w].as_is();
            chval.push(mydp2[i].clone());
        }
        dp_loo[v] = LOO::build(me[v].clone(), &chval, app);
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

// Depends on: tree/Reroot.rs
#[derive(Default, Clone)]
struct NumeralConcatenation {
    // the multiplier that would be multiplied by the value of the parent node
    mul: MInt,
    // the sum in the current rooted tree
    add: MInt,
    // the info of the root node
    me: (MInt, MInt),
}

impl LeaveOne for NumeralConcatenation {
    type T = (MInt, MInt);
    type Me = (MInt, MInt);
    type App = ();
    fn build(me: Self::Me, vals: &[Self::T], &(): &Self::App) -> Self {
        let mut mul = MInt::new(0);
        let mut add = MInt::new(0);
        for &(b, d) in vals {
            mul += b;
            add += d;
        }
        NumeralConcatenation {
            mul: (mul + 1) * me.0,
            add: add + me.1 * (mul + 1),
            me: me,
        }
    }
    fn leave_one(&self, (b, d): Self::T) -> Self::T {
        (self.mul - self.me.0 * b, self.add - self.me.1 * b - d)
    }
    fn exchange_one(&self, (b1, d1): Self::T, (b2, d2): Self::T) -> Self::T {
        (self.mul - (b1 - b2) * self.me.0, self.add - (b1 - b2) * self.me.1 - d1 + d2)
    }
    fn add_one(&self, (b, d): Self::T) -> Self::T {
        (self.mul + self.me.0 * b, self.add + self.me.1 * b + d)
    }
    fn as_is(&self) -> Self::T {
        (self.mul, self.add)
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// https://yukicoder.me/problems/no/2360 (3)
// 全方位木 DP。
// Tags: rerooting
fn solve() {
    input! {
        n: usize,
        a: [chars; n],
        uv: [(usize1, usize1); n - 1],
    }
    let mut t = vec![];
    for i in 0..n {
        let mut mul = MInt::new(1);
        let mut add = MInt::new(0);
        for &c in &a[i] {
            mul *= 10;
            add = add * 10 + (c as u8 - b'0') as i64;
        }
        t.push((mul, add));
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &uv {
        g[a].push(b);
        g[b].push(a);
    }
    let reroot = Reroot::<NumeralConcatenation>::new(&g, &t, &());
    let mut ans = MInt::new(0);
    for i in 0..n {
        let (_mul, add) = reroot.dp1[i];
        ans += add;
    }
    println!("{}", ans);
}
