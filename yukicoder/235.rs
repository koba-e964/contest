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

#[allow(dead_code)]
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
const MOD: i64 = 1_000_000_007;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

// https://ei1333.github.io/luzhiled/snippets/tree/heavy-light-decomposition.html
// Verified by: NUPC2017 H
// https://atcoder.jp/contests/njpc2017/submissions/23535017
struct HLDecomp {
    euler: Vec<usize>,
    head: Vec<usize>,
    rev: Vec<usize>,
    par: Vec<usize>,
}

impl HLDecomp {
    fn dfs_sz(v: usize, p: usize, g: &mut [Vec<usize>], sz: &mut [usize],
              par: &mut [usize]) {
        par[v] = p;
        sz[v] = 1;
        if g[v].get(0) == Some(&p) {
            let last = g[v].len() - 1;
            g[v].swap(0, last);
        }
        for i in 0..g[v].len() {
            let to = g[v][i];
            if to == p {
                continue;
            }
            Self::dfs_sz(to, v, g, sz, par);
            sz[v] += sz[to];
            if sz[g[v][0]] < sz[to] {
                g[v].swap(0, i);
            }
        }
    }
    fn dfs_euler(v: usize, par: usize, g: &[Vec<usize>],
                 euler: &mut [usize], count: &mut usize,
                 head: &mut [usize], rev: &mut [usize]) {
        euler[v] = *count;
        *count += 1;
        rev[euler[v]] = v;
        for &to in &g[v] {
            if to == par {
                continue;
            }
            head[to] = if g[v][0] == to { head[v] } else { to };
            Self::dfs_euler(to, v, g, euler, count, head, rev);
        }
    }
    pub fn new(g: &[Vec<usize>]) -> Self {
        let mut g = g.to_vec();
        let n = g.len();
        let mut sz = vec![0; n];
        let mut par = vec![0; n];
        Self::dfs_sz(0, n, &mut g, &mut sz, &mut par);
        let mut euler = vec![0; n];
        let mut count = 0;
        let mut head = vec![0; n];
        let mut rev = vec![0; n];
        Self::dfs_euler(0, n, &g, &mut euler, &mut count, &mut head, &mut rev);
        HLDecomp {
            euler: euler,
            head: head,
            rev: rev,
            par: par,
        }
    }
    #[allow(unused)]
    pub fn get_id(&self, v: usize) -> usize {
        self.euler[v]
    }
    #[allow(unused)]
    pub fn from_id(&self, id: usize) -> usize {
        self.rev[id]
    }
    // M: commutative
    // M must not panic.
    #[allow(unused)]
    pub fn query<T, F: FnMut(usize, usize) -> T, M: Fn(T, T) -> T>(&self, mut u: usize, mut v: usize, mut f: F, mut m: M, e: T, edge: bool) -> T {
        let mut ans = e;
        self.divide(u, v, |l, r| {
            let ptr: *mut T = &mut ans;
            unsafe {
                let val = f(l, r);
                let ans = std::ptr::read(ptr);
                std::ptr::write(ptr, m(ans, val))
            }
        }, edge);
        ans
    }
    pub fn divide<F: FnMut(usize, usize)>(&self, mut u: usize, mut v: usize, mut f: F, edge: bool) {
        let euler = &self.euler;
        let head = &self.head;
        loop {
            if euler[u] > euler[v] {
                std::mem::swap(&mut u, &mut v);
            }
            if head[u] == head[v] {
                break;
            }
            f(euler[head[v]], euler[v] + 1);
            v = self.par[head[v]];
        }
        f(euler[u] + if edge { 1 } else { 0 }, euler[v] + 1);
    }
}

/**
 * Lazy Segment Tree. This data structure is useful for fast folding and updating on intervals of an array
 * whose elements are elements of monoid T. Note that constructing this tree requires the identity
 * element of T and the operation of T. This is monomorphised, because of efficiency. T := i64, biop = max, upop = (+)
 * Reference: http://d.hatena.ne.jp/kyuridenamida/20121114/1352835261
 * Verified by https://codeforces.com/contest/1114/submission/49759034
 */
pub trait ActionRing {
    type T: Clone + Copy; // data
    type U: Clone + Copy + PartialEq + Eq; // action
    fn biop(x: Self::T, y: Self::T) -> Self::T;
    fn update(x: Self::T, a: Self::U, height: usize) -> Self::T;
    fn upop(fst: Self::U, snd: Self::U) -> Self::U;
    fn e() -> Self::T;
    fn upe() -> Self::U; // identity for upop
}
pub struct LazySegTree<R: ActionRing> {
    n: usize,
    dep: usize,
    dat: Vec<R::T>,
    lazy: Vec<R::U>,
}

impl<R: ActionRing> LazySegTree<R> {
    #[allow(unused)]
    pub fn new(n_: usize) -> Self {
        let mut n = 1;
        let mut dep = 0;
        while n < n_ { n *= 2; dep += 1; } // n is a power of 2
        LazySegTree {
            n: n,
            dep: dep,
            dat: vec![R::e(); 2 * n - 1],
            lazy: vec![R::upe(); 2 * n - 1]
        }
    }
    #[allow(unused)]
    pub fn with(a: &[R::T]) -> Self {
        let n_ = a.len();
        let mut n = 1;
        let mut dep = 0;
        while n < n_ { n *= 2; dep += 1; } // n is a power of 2
        let mut dat = vec![R::e(); 2 * n - 1];
        for i in 0..n_ {
            dat[n - 1 + i] = a[i];
        }
        for i in (0..n - 1).rev() {
            dat[i] = R::biop(dat[2 * i + 1], dat[2 * i + 2]);
        }
        LazySegTree {
            n: n,
            dep: dep,
            dat: dat,
            lazy: vec![R::upe(); 2 * n - 1],
        }
    }
    #[inline]
    fn lazy_evaluate_node(&mut self, k: usize, height: usize) {
        if self.lazy[k] == R::upe() { return; }
        self.dat[k] = R::update(self.dat[k], self.lazy[k], height);
        if k < self.n - 1 {
            self.lazy[2 * k + 1] = R::upop(self.lazy[2 * k + 1], self.lazy[k]);
            self.lazy[2 * k + 2] = R::upop(self.lazy[2 * k + 2], self.lazy[k]);
        }
        self.lazy[k] = R::upe(); // identity for upop
    }
    #[inline]
    fn update_node(&mut self, k: usize) {
        self.dat[k] = R::biop(self.dat[2 * k + 1], self.dat[2 * k + 2]);
    }
    fn update_sub(&mut self, a: usize, b: usize, v: R::U, k: usize, height: usize, l: usize, r: usize) {
        self.lazy_evaluate_node(k, height);

        // [a,b) and  [l,r) intersects?
        if r <= a || b <= l {return;}
        if a <= l && r <= b {
            self.lazy[k] = R::upop(self.lazy[k], v);
            self.lazy_evaluate_node(k, height);
            return;
        }

        self.update_sub(a, b, v, 2 * k + 1, height - 1, l, (l + r) / 2);
        self.update_sub(a, b, v, 2 * k + 2, height - 1, (l + r) / 2, r);
        self.update_node(k);
    }
    /* ary[i] = upop(ary[i], v) for i in [a, b) (half-inclusive) */
    #[inline]
    pub fn update(&mut self, a: usize, b: usize, v: R::U) {
        let n = self.n;
        let dep = self.dep;
        self.update_sub(a, b, v, 0, dep, 0, n);
    }
    /* l,r are for simplicity */
    fn query_sub(&mut self, a: usize, b: usize, k: usize, height: usize, l: usize, r: usize) -> R::T {
        self.lazy_evaluate_node(k, height);

        // [a,b) and  [l,r) intersect?
        if r <= a || b <= l {return R::e();}
        if a <= l && r <= b {return self.dat[k];}
        let vl = self.query_sub(a, b, 2 * k + 1, height - 1, l, (l + r) / 2);
        let vr = self.query_sub(a, b, 2 * k + 2, height - 1, (l + r) / 2, r);
        self.update_node(k);
        R::biop(vl, vr)
    }
    /* [a, b) (note: half-inclusive) */
    #[inline]
    pub fn query(&mut self, a: usize, b: usize) -> R::T {
        let n = self.n;
        let dep = self.dep;
        self.query_sub(a, b, 0, dep, 0, n)
    }
}

enum VFix {}

const B: usize = 2;

impl ActionRing for VFix {
    type T = [MInt; B]; // data
    type U = [MInt; B]; // action, [[a, 0], [b, 1]]
    fn biop(x: Self::T, y: Self::T) -> Self::T {
        let mut ans = [0.into(); B];
        for i in 0..B {
            ans[i] = x[i] + y[i];
        }
        ans
    }
    fn update(x: Self::T, o: Self::U, _height: usize) -> Self::T {
        let mut ans = [0.into(); B];
        for i in 0..B {
            ans[0] += x[i] * o[i];
        }
        ans[1] = x[1];
        ans
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        let mut ans = [0.into(); B];
        for i in 0..B {
            ans[i] += fst[i] * snd[0];
        }
        ans[1] += snd[1];
        ans
    }
    fn e() -> Self::T {
        [0.into(); B]
    }
    fn upe() -> Self::U { // identity for upop
        [1.into(), 0.into()]
    }
}

fn solve() {
    let n: usize = get();
    let s: Vec<i64> = (0..n).map(|_| get()).collect();
    let c: Vec<i64> = (0..n).map(|_| get()).collect();
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        g[a].push(b);
        g[b].push(a);
    }
    let hld = HLDecomp::new(&g);
    let mut init = vec![[MInt::new(0); B]; n];
    for i in 0..n {
        let idx = hld.get_id(i);
        init[idx] = [s[i].into(), c[i].into()];
    }
    let mut st = LazySegTree::<VFix>::with(&init);
    let q: usize = get();
    for _ in 0..q {
        let ty: i32 = get();
        let x = get::<usize>() - 1;
        let y = get::<usize>() - 1;
        if ty == 0 {
            let z: i64 = get();
            hld.divide(x, y, |l, r| st.update(l, r, [1.into(), z.into()]), false);
        } else {
            let mut tot = MInt::new(0);
            hld.divide(x, y, |l, r| tot += st.query(l, r)[0], false);
            println!("{}", tot);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
