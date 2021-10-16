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
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Verified by: yukicoder No. 259 (http://yukicoder.me/submissions/100581)
 *              AGC015-E (http://agc015.contest.atcoder.jp/submissions/1461001)
 *              yukicoder No. 833 (https://yukicoder.me/submissions/703521)
 */
struct SegTree<I, BiOp> {
    n: usize,
    dat: Vec<I>,
    op: BiOp,
    e: I,
}

impl<I, BiOp> SegTree<I, BiOp>
    where BiOp: Fn(I, I) -> I,
          I: Copy {
    pub fn new(n_: usize, op: BiOp, e: I) -> Self {
        let mut n = 1;
        while n < n_ { n *= 2; } // n is a power of 2
        SegTree {n: n, dat: vec![e; 2 * n - 1], op: op, e: e}
    }
    /* ary[k] <- v */
    pub fn update(&mut self, idx: usize, v: I) {
        let mut k = idx + self.n - 1;
        self.dat[k] = v;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.op)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
        }
    }
    /* [a, b) (note: half-inclusive)
     * http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/ */
    #[allow(unused)]
    pub fn query(&self, mut a: usize, mut b: usize) -> I {
        let mut left = self.e;
        let mut right = self.e;
        a += self.n - 1;
        b += self.n - 1;
        while a < b {
            if (a & 1) == 0 {
                left = (self.op)(left, self.dat[a]);
            }
            if (b & 1) == 0 {
                right = (self.op)(self.dat[b - 1], right);
            }
            a = a / 2;
            b = (b - 1) / 2;
        }
        (self.op)(left, right)
    }
    // Port from https://github.com/atcoder/ac-library/blob/master/atcoder/segtree.hpp
    #[allow(unused)]
    fn max_right<F: Fn(I) -> bool>(
        &self, mut l: usize, f: &F,
    ) -> usize {
        assert!(f(self.e));
        if l == self.n {
            return self.n;
        }
        l += self.n - 1;
        let mut sm = self.e;
        loop {
            while l % 2 == 1 {
                l = (l - 1) / 2;
            }
            if !f((self.op)(sm, self.dat[l])) {
                while l < self.n - 1 {
                    l = 2 * l + 1;
                    let val = (self.op)(sm, self.dat[l]);
                    if f(val) {
                        sm = val;
                        l += 1;
                    }
                }
                return l + 1 - self.n;
            }
            sm = (self.op)(sm, self.dat[l]);
            l += 1;
            if (l + 1).is_power_of_two() { break; }
        }
        self.n
    }
    // Port from https://github.com/atcoder/ac-library/blob/master/atcoder/segtree.hpp
    #[allow(unused)]
    fn min_left<F: Fn(I) -> bool>(
        &self, mut r: usize, f: &F,
    ) -> usize {
        if !f(self.e) {
            return r + 1;
        }
        if r == 0 {
            return 0;
        }
        r += self.n - 1;
        let mut sm = self.e;
        loop {
            r -= 1;
            while r > 0 && r % 2 == 0 {
                r = (r - 1) / 2;
            }
            if !f((self.op)(self.dat[r], sm)) {
                while r < self.n - 1 {
                    r = 2 * r + 2;
                    let val = (self.op)(self.dat[r], sm);
                    if f(val) {
                        sm = val;
                        r -= 1;
                    }
                }
                return r + 2 - self.n;
            }
            sm = (self.op)(self.dat[r], sm);
            if (r + 1).is_power_of_two() { break; }
        }
        0
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

fn mul(x: [[MInt; 2]; 2], y: [[MInt; 2]; 2]) -> [[MInt; 2]; 2] {
    let mut z = [[MInt::new(0); 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                z[i][k] += x[i][j] * y[j][k];
            }
        }
    }
    z
}

fn solve() {
    let n: usize = get();
    let mut g = vec![vec![]; n];
    let mut edge = vec![];
    for _ in 0..n - 1 {
        let a: usize = get();
        let b: usize = get();
        g[a].push(b);
        g[b].push(a);
        edge.push((a, b));
    }
    let mut st = SegTree::new(n, mul, [[MInt::new(1), MInt::new(0)], [MInt::new(0), MInt::new(1)]]);
    let hld = HLDecomp::new(&g);
    let q: usize = get();
    for _ in 0..q {
        let ty: String = get_word();
        if ty == "x" {
            let i: usize = get();
            let (a, b) = edge[i];
            let mut f = [[MInt::new(0); 2]; 2];
            for j in 0..2 {
                for k in 0..2 {
                    let x: i64 = get();
                    f[j][k] = x.into();
                }
            }
            let mut eidx = n;
            hld.divide(a, b, |l, r| if l < r { eidx = l }, true);
            st.update(eidx, f);
        } else {
            let a: usize = get();
            let b: usize = get();
            let mut tmp = st.e;
            hld.divide(a, b, |l, r| tmp = mul(st.query(l, r), tmp), true);
            for i in 0..4 {
                print!("{}{}", tmp[i / 2][i % 2], if i == 3 { "\n" } else { " " });
            }
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
