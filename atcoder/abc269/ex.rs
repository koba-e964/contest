use std::cmp::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Plain HL decomposition.
pub struct PlainHLD {
    pub st: Vec<usize>,
    pub jmp: Vec<usize>,
    pub dep: Vec<usize>,
}

impl PlainHLD {
    // For each node, make the most heavy child the first child.
    fn dfs_left(ch: &mut [Vec<usize>], v: usize, sz: &mut [usize],
                dep: &mut [usize]) {
        let mut stack = vec![(v, 0, 0)];
        while let Some((v, d, kind)) = stack.pop() {
            if kind == 0 {
                dep[v] = d;
                stack.push((v, d, 1));
                for i in 0..ch[v].len() {
                    let w = ch[v][i];
                    stack.push((w, d + 1, 0));
                }
            } else {
                let mut s = 1;
                for i in 0..ch[v].len() {
                    let w = ch[v][i];
                    s += sz[w];
                    if sz[w] > sz[ch[v][0]] {
                        ch[v].swap(i, 0);
                    }
                }
                sz[v] = s;
            }
        }
    }
    fn dfs(ch: &[Vec<usize>], st: &mut [usize], v: usize, jmp: &mut [usize]) {
        let mut stack = vec![v];
        let mut cnt = 0;
        while let Some(v) = stack.pop() {
            st[v] = cnt;
            cnt += 1;
            if ch[v].len() >= 1 {
                jmp[ch[v][0]] = jmp[v];
            }
            for &w in &ch[v] {
                stack.push(w);
            }
        }
    }
    pub fn new(ch: &mut [Vec<usize>], root: usize) -> Self {
        let n = ch.len();
        let mut st = vec![0; n];
        let mut sz = vec![0; n];
        let mut jmp = vec![0; n];
        let mut dep = vec![0; n];
        Self::dfs_left(ch, root, &mut sz, &mut dep);
        for i in 0..n {
            jmp[i] = i;
        }
        Self::dfs(ch, &mut st, root, &mut jmp);
        PlainHLD {
            st: st,
            jmp: jmp,
            dep: dep,
        }
    }
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

// FFT (in-place, verified as NTT only)
// R: Ring + Copy
// Verified by: https://judge.yosupo.jp/submission/53831
// Adopts the technique used in https://judge.yosupo.jp/submission/3153.
mod fft {
    use std::ops::*;
    // n should be a power of 2. zeta is a primitive n-th root of unity.
    // one is unity
    // Note that the result is bit-reversed.
    pub fn fft<R>(f: &mut [R], zeta: R, one: R)
        where R: Copy +
        Add<Output = R> +
        Sub<Output = R> +
        Mul<Output = R> {
        let n = f.len();
        assert!(n.is_power_of_two());
        let mut m = n;
        let mut base = zeta;
        unsafe {
            while m > 2 {
                m >>= 1;
                let mut r = 0;
                while r < n {
                    let mut w = one;
                    for s in r..r + m {
                        let &u = f.get_unchecked(s);
                        let d = *f.get_unchecked(s + m);
                        *f.get_unchecked_mut(s) = u + d;
                        *f.get_unchecked_mut(s + m) = w * (u - d);
                        w = w * base;
                    }
                    r += 2 * m;
                }
                base = base * base;
            }
            if m > 1 {
                // m = 1
                let mut r = 0;
                while r < n {
                    let &u = f.get_unchecked(r);
                    let d = *f.get_unchecked(r + 1);
                    *f.get_unchecked_mut(r) = u + d;
                    *f.get_unchecked_mut(r + 1) = u - d;
                    r += 2;
                }
            }
        }
    }
    pub fn inv_fft<R>(f: &mut [R], zeta_inv: R, one: R)
        where R: Copy +
        Add<Output = R> +
        Sub<Output = R> +
        Mul<Output = R> {
        let n = f.len();
        assert!(n.is_power_of_two());
        let zeta = zeta_inv; // inverse FFT
        let mut zetapow = Vec::with_capacity(20);
        {
            let mut m = 1;
            let mut cur = zeta;
            while m < n {
                zetapow.push(cur);
                cur = cur * cur;
                m *= 2;
            }
        }
        let mut m = 1;
        unsafe {
            if m < n {
                zetapow.pop();
                let mut r = 0;
                while r < n {
                    let &u = f.get_unchecked(r);
                    let d = *f.get_unchecked(r + 1);
                    *f.get_unchecked_mut(r) = u + d;
                    *f.get_unchecked_mut(r + 1) = u - d;
                    r += 2;
                }
                m = 2;
            }
            while m < n {
                let base = zetapow.pop().unwrap();
                let mut r = 0;
                while r < n {
                    let mut w = one;
                    for s in r..r + m {
                        let &u = f.get_unchecked(s);
                        let d = *f.get_unchecked(s + m) * w;
                        *f.get_unchecked_mut(s) = u + d;
                        *f.get_unchecked_mut(s + m) = u - d;
                        w = w * base;
                    }
                    r += 2 * m;
                }
                m *= 2;
            }
        }
    }
}

// Depends on: fft.rs, MInt.rs
pub struct FPSOps<M: mod_int::Mod> {
    gen: mod_int::ModInt<M>,
}

impl<M: mod_int::Mod> FPSOps<M> {
    pub fn new(gen: mod_int::ModInt<M>) -> Self {
        FPSOps { gen: gen }
    }
}

impl<M: mod_int::Mod> FPSOps<M> {
    pub fn add(&self, mut a: Vec<mod_int::ModInt<M>>, mut b: Vec<mod_int::ModInt<M>>) -> Vec<mod_int::ModInt<M>> {
        if a.len() < b.len() {
            std::mem::swap(&mut a, &mut b);
        }
        for i in 0..b.len() {
            a[i] += b[i];
        }
        a
    }
    pub fn mul(&self, a: Vec<mod_int::ModInt<M>>, b: Vec<mod_int::ModInt<M>>) -> Vec<mod_int::ModInt<M>> {
        type MInt<M> = mod_int::ModInt<M>;
        let n = a.len() - 1;
        let m = b.len() - 1;
        let mut p = 1;
        while p <= n + m { p *= 2; }
        let mut f = vec![MInt::new(0); p];
        let mut g = vec![MInt::new(0); p];
        for i in 0..n + 1 { f[i] = a[i]; }
        for i in 0..m + 1 { g[i] = b[i]; }
        let fac = MInt::new(p as i64).inv();
        let zeta = self.gen.pow((M::m() - 1) / p as i64);
        fft::fft(&mut f, zeta, 1.into());
        fft::fft(&mut g, zeta, 1.into());
        for i in 0..p { f[i] *= g[i] * fac; }
        fft::inv_fft(&mut f, zeta.inv(), 1.into());
        f.truncate(n + m + 1);
        f
    }
}

// O(n log^2 n + args.len()) where n = sum(degs of args)
fn multiply_all(ops: &FPSOps<P>, args: Vec<Vec<MInt>>) -> Vec<MInt> {
    #[derive(Eq, PartialEq)]
    struct T(usize, Vec<MInt>);
    impl Ord for T {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.0.cmp(&self.0)
        }
    }
    impl PartialOrd for T {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    let mut polys = BinaryHeap::new();
    for a in args {
        polys.push(T(a.len(), a));
    }
    let mut ans = vec![MInt::new(1)];
    while let Some(T(_, t1)) = polys.pop() {
        let t2 = if let Some(T(_, t)) = polys.pop() {
            t
        } else {
            ans = t1;
            break;
        };
        let tmp = ops.mul(t1, t2);
        polys.push(T(tmp.len(), tmp));
    }
    ans
}

fn accumulate_and_add(ops: &FPSOps<P>, mut args: Vec<Vec<MInt>>) -> Vec<MInt> {
    fn inner(ops: &FPSOps<P>, args: &mut [Vec<MInt>], l: usize, r: usize) -> (Vec<MInt>, Vec<MInt>) {
        if r - l == 1 {
            return (std::mem::replace(&mut args[l], vec![]), vec![0.into(), 1.into()]);
        }
        let mut degsum = 0;
        for i in l..r {
            degsum += args[i].len();
        }
        let mut m = l;
        let mut tmp = 0;
        while m < r - 1 && tmp * 2 < degsum {
            tmp += args[m].len();
            m += 1;
        }
        let (rp, rs) = inner(ops, args, m, r);
        let (lp, ls) = inner(ops, args, l, m);
        let sum = ops.add(ops.mul(ls, rp.clone()), rs);
        (ops.mul(lp, rp), sum)
    }
    let l = args.len();
    let (a, b) = inner(ops, &mut args, 0, l);
    ops.add(a, b)
}

// The author read the editorial before implementing this.
// Tags: heavy-light-decomposition, hl-decomposition, divide-and-conquer-fft
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        p: [usize; n - 1],
    }
    let mut ch = vec![vec![]; n];
    for i in 0..n - 1 {
        ch[p[i] - 1].push(i + 1);
    }
    let fps = FPSOps::new(MInt::new(3));
    let hld = PlainHLD::new(&mut ch, 0);
    let mut heavy = vec![vec![]; n];
    let mut light = vec![vec![]; n];
    for i in (0..n).rev() {
        let mut old = vec![];
        if !ch[i].is_empty() {
            old = std::mem::replace(&mut heavy[ch[i][0]], vec![]);
        }
        let mut chs = vec![];
        for j in 1..ch[i].len() {
            chs.push(std::mem::replace(&mut light[ch[i][j]], vec![]));
        }
        let x = multiply_all(&fps, chs);
        old.push(x);
        if hld.jmp[i] == i {
            light[i] = accumulate_and_add(&fps, old);
        } else {
            heavy[i] = old;
        }
    }
    for i in 1..n + 1 {
        puts!("{}\n", if i < light[0].len() { light[0][i] } else { 0.into() });
    }
}
