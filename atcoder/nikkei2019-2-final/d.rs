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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
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

struct Rng {
    x: u64,
}

impl Rng {
    fn new() -> Self {
        use std::hash::{Hasher, BuildHasher};
        let hm = std::collections::HashMap::<i32, i32>::new();
        let mut hash = hm.hasher().build_hasher();
        hash.write_u32(8128);
        Rng {
            x: hash.finish(),
        }
    }
    fn next(&mut self) -> u64 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        x ^ x << 10
    }
}

fn find_subtree_sizes(g: &[Vec<usize>], v: usize, par: usize,
                      dp: &mut [usize]) {
    let mut sum = 1;
    for &w in &g[v] {
        if par == w { continue; }
        find_subtree_sizes(g, w, v, dp);
        sum += dp[w];
    }
    dp[v] = sum;
}

fn find_centroid(g: &[Vec<usize>]) -> Result<usize, (usize, usize)> {
    let n = g.len();
    let v = 0;
    let mut dp = vec![0; n];
    find_subtree_sizes(g, v, n, &mut dp);
    {
        let mut v = v;
        let mut par = n;
        loop {
            let mut has_majority = false;
            let mut has_half = None;
            for &w in &g[v] {
                if par == w { continue; }
                if dp[w] * 2 == n {
                    has_half = Some(w);
                }
                if dp[w] > n / 2 {
                    par = v;
                    v = w;
                    has_majority = true;
                    break;
                }
            }
            if !has_majority {
                if let Some(w) = has_half {
                    return Err((w, v));
                }
                return Ok(v);
            }
        }
    }
}

const B: usize = 2;

// https://snuke.hatenablog.com/entry/2017/02/03/054210
fn get_hash(v: usize, par: usize, dep: usize,
            g: &[Vec<usize>], base: &[[MInt; B]]) -> [MInt; B] {
    let mut ans = [MInt::new(1); B];
    for &w in &g[v] {
        if w == par {
            continue;
        }
        let sub = get_hash(w, v, dep + 1, g, base);
        for i in 0..B {
            ans[i] *= base[dep][i] + sub[i];
        }
    }
    ans
}

fn get_hash_1_sub(
    v: usize, par: usize, dep: usize,
    g: &[Vec<usize>], base: &[[MInt; B]],
) -> ([MInt; B], [MInt; B], [MInt; B], Vec<(usize, [MInt; B])>) {
    let mut ans = [MInt::new(1); B];
    let mut a = [MInt::new(1); B];
    let mut b = [MInt::new(0); B];
    let mut m1 = vec![];
    for &w in &g[v] {
        if w == par {
            continue;
        }
        let (mut subans, mut suba, mut subb, mut subm1) = if g[w].len() == 1 {
            let mut subans = [MInt::new(0); B];
            let suba = [MInt::new(1); B];
            let subb = [MInt::new(0); B];
            let subm1 = vec![(w, [MInt::new(1); B])];
            for i in 0..B {
                subans[i] = base[dep][i] + 1;
            }
            (subans, suba, subb, subm1)
        } else {
            let (mut subans, suba, mut subb, subm1) = get_hash_1_sub(w, v, dep + 1, g, base);
            for i in 0..B {
                subans[i] += base[dep][i];
                subb[i] += base[dep][i];
            }
            (subans, suba, subb, subm1)
        };
        if m1.len() < subm1.len() {
            std::mem::swap(&mut ans, &mut subans);
            std::mem::swap(&mut a, &mut suba);
            std::mem::swap(&mut b, &mut subb);
            std::mem::swap(&mut m1, &mut subm1);
        }
        let mut tmp1 = [MInt::new(0); B];
        let mut tmp2 = [MInt::new(0); B];
        for i in 0..B {
            assert_ne!(a[i], 0.into());
            assert_ne!(subans[i], 0.into());
            tmp1[i] = ans[i] * subans[i].inv();
            tmp2[i] = a[i].inv();
        }
        for (idx, mut t) in subm1 {
            for i in 0..B {
                t[i] = (suba[i] * t[i] + subb[i]) * tmp1[i] - b[i];
                t[i] *= tmp2[i];
            }
            m1.push((idx, t));
        }
        for i in 0..B {
            a[i] *= subans[i];
            b[i] *= subans[i];
            ans[i] *= subans[i];
        }
    }
    (ans, a, b, m1)
}

fn get_hash_1(
    root: usize,
    g: &[Vec<usize>], base: &[[MInt; B]],
) -> Vec<(usize, [MInt; B])> {
    let n = g.len();
    let (_, a, b, mut m1) = get_hash_1_sub(root, n, 0, g, base);
    for x in &mut m1 {
        for i in 0..B {
            x.1[i] = a[i] * x.1[i] + b[i];
        }
    }
    m1
}

// Tags: centroid, tree-hashing, merging-sets
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        v: [usize1; n],
        u: [usize1; n - 1],
    }
    let mut g0 = vec![vec![]; n + 1];
    let mut g1 = vec![vec![]; n];
    for i in 0..n {
        g0[i].push(v[i]);
        g0[v[i]].push(i);
    }
    for i in 0..n - 1 {
        g1[i].push(u[i]);
        g1[u[i]].push(i);
    }
    let c0 = find_centroid(&g0);
    let c1 = find_centroid(&g1);
    let mut base = vec![[MInt::new(0); B]; n];
    let mut rng = Rng::new();
    for i in 0..n {
        for j in 0..B {
            base[i][j] += (rng.next() >> 32) as i64;
        }
    }
    let hash1 = match c1 {
        Ok(v) => vec![get_hash(v, n, 0, &g1, &base)],
        Err((w, v)) => {
            let s0 = get_hash(w, n, 0, &g1, &base);
            let s1 = get_hash(v, n, 0, &g1, &base);
            vec![s0, s1]
        }
    };
    let mut ans = vec![];
    let sub = match c0 {
        Ok(v) => {
            vec![get_hash_1(v, &g0, &base)]
        }
        Err((w, v)) => {
            vec![
                get_hash_1(w, &g0, &base),
                get_hash_1(v, &g0, &base),
            ]
        }
    };
    if false {
        eprintln!("{:?}, {:?}", c0, c1);
        eprintln!("hash1 = {:?}", hash1);
        eprintln!("sub = {:?}", sub);
    }
    for sub in sub {
        for (idx, m1) in sub {
            if hash1.iter().any(|&h| h == m1) {
                ans.push(idx + 1);
            }
        }
    }
    ans.sort(); ans.dedup();
    for i in 0..ans.len() {
        puts!("{}{}", ans[i], if i + 1 == ans.len() { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
