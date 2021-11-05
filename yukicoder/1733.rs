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
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

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

enum Affine {}

impl ActionRing for Affine {
    type T = MInt; // data
    type U = (MInt, MInt); // action, (a, b) |-> x |-> ax + b
    fn biop(x: Self::T, y: Self::T) -> Self::T {
        x + y
    }
    fn update(x: Self::T, (a, b): Self::U, height: usize) -> Self::T {
        x * a + b * MInt::new(1 << height)
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        let (a, b) = fst;
        let (c, d) = snd;
        (a * c, b * c + d)
    }
    fn e() -> Self::T {
        0.into()
    }
    fn upe() -> Self::U { // identity for upop
        (1.into(), 0.into())
    }
}

fn find_coef(a: &[i64], eq: bool) -> Vec<MInt> {
    let n = a.len();
    let mut b = vec![];
    for i in 0..n {
        b.push((a[i], i));
    }
    if eq {
        b.sort();
    } else {
        b.sort_by_key(|&(a, b)| (a, n - b));
    }
    let mut st = LazySegTree::<Affine>::new(n);
    st.update(0, n, (0.into(), 1.into()));
    let mut ans = vec![MInt::new(1); n];
    for (_, idx) in b {
        let p = st.query(0, idx + 1);
        let q = st.query(idx, idx + 1);
        ans[idx] = p * q.inv();
        st.update(0, idx + 1, (2.into(), 0.into()));
    }
    ans
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let p = find_coef(&a, true);
    let mut a = a;
    a.reverse();
    let mut q = find_coef(&a, false);
    q.reverse();
    a.reverse();
    let mut tot = MInt::new(0);
    for i in 0..n {
        tot += p[i] * q[i] * a[i];
    }
    println!("{}", tot);
}
