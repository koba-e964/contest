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
const MOD: i64 = 1_000_000_007;
define_mod!(P, MOD);
type ModInt = mod_int::ModInt<P>;

// Depends on ModInt.rs
fn fact_init(w: usize) -> (Vec<ModInt>, Vec<ModInt>) {
    let mut fac = vec![ModInt::new(1); w];
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

// Rerooting
struct Reroot<T> {
    g: Vec<Vec<usize>>,
    zero: T,
    dp1: Vec<T>,
    dp2: Vec<T>,
    ch: Vec<Vec<usize>>,
    acc_l: Vec<Vec<T>>,
    acc_r: Vec<Vec<T>>,
}

impl<T: Copy> Reroot<T> {
    pub fn new(g: &[Vec<usize>], zero: T) -> Reroot<T> {
        let n = g.len();
        Reroot {
            g: g.to_vec(),
            zero: zero.clone(),
            dp1: vec![zero.clone(); n],
            dp2: vec![zero.clone(); n],
            ch: vec![vec![]; n],
            acc_l: vec![vec![]; n],
            acc_r: vec![vec![]; n],
        }
    }
    // TODO include f in struct
    pub fn do_comp<F: FnMut(T, T) -> T, D: FnMut(T) -> T>(
        &mut self,
        f: &mut F,
        d: &mut D,
    ) {
        let n = self.g.len();
        Self::dfs1(0, n, &self.g, &mut self.dp1, &mut self.ch,
                   &mut self.acc_l, &mut self.acc_r, self.zero, f, d);
        Self::dfs2(0, &self.ch, &self.dp1, &mut self.dp2,
                   self.zero,
                   &self.acc_l, &self.acc_r, self.zero, f, d);
    }
    fn dfs1<F: FnMut(T, T) -> T, D: FnMut(T) -> T>(
        v: usize, par: usize, g: &[Vec<usize>],
        dp1: &mut [T],
        ch: &mut [Vec<usize>],
        acc_l: &mut [Vec<T>], acc_r: &mut [Vec<T>],
        zero: T,
        f: &mut F,
        d: &mut D,
    ) {
        let mut ary = vec![];
        let mut mych = vec![];
        for &w in &g[v] {
            if w == par { continue; }
            mych.push(w);
            Self::dfs1(w, v, g, dp1, ch, acc_l, acc_r, zero.clone(), f, d);
            ary.push(dp1[w].clone());
        }
        let m = ary.len();
        acc_l[v] = vec![zero.clone(); m + 1];
        acc_r[v] = vec![zero.clone(); m + 1];
        for i in 0..m {
            let val = f(acc_l[v][i], ary[i]);
            acc_l[v][i + 1] = val;
        }
        for i in (0..m).rev() {
            let val = f(acc_r[v][i + 1], ary[i]);
            acc_r[v][i] = val;
        }
        ch[v] = mych;
        dp1[v] = d(acc_r[v][0]);
    }
    fn dfs2<F: FnMut(T, T) -> T, D: FnMut(T) -> T>(
        v: usize, ch: &[Vec<usize>],
        dp1: &[T],
        dp2: &mut [T],
        passed: T,
        acc_l: &[Vec<T>], acc_r: &[Vec<T>],
        zero: T,
        f: &mut F,
        d: &mut D,
    ) {
        dp2[v] = passed;
        for i in 0..ch[v].len() {
            let w = ch[v][i];
            let leave_one = f(acc_l[v][i], acc_r[v][i + 1]);
            Self::dfs2(w, ch, dp1, dp2, d(f(leave_one, passed)), acc_l, acc_r,
                       zero, f, d);
        }
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
    }
    let (fac, invfac) = fact_init(n + 1);
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut reroot = Reroot::new(&g, (ModInt::new(1), 0));
    let mut f = |(x, a), (y, b)| {
        (x * y * invfac[a] * invfac[b] * fac[a + b], a + b)
    };
    reroot.do_comp(&mut f,
                   &mut |(x, a)| (x, a + 1));
    puts!("{}\n", reroot.dp1[0].0);
    for i in 1..n {
        let tmp = f(reroot.acc_r[i][0], reroot.dp2[i]).0;
        puts!("{}\n", tmp);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
