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
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

/**
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
 */
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![1; n] }
    }
    fn root(&mut self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        self.disj[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.disj[x] = y;
        self.rank[y] += self.rank[x];
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.rank[x]
    }
}

// Finds ~ where x ~ y := {i | x in a[i] } = {i | y in a[i] }
// Returns a sequence consisting of (equiv class, {i | x in a[i] })
// Complexity: O(nw + n log n)
#[allow(unused)]
fn decompose_into_equivalence_classes(a: &[u128]) -> Vec<(u128, u128)> {
    let mut pp = vec![];
    let mut to = vec![];
    for i in 0..128 {
        let mut tmp = 0;
        for j in 0..a.len() {
            if (a[j] & 1 << i) != 0 {
                tmp |= 1 << j;
            }
        }
        if tmp != 0 {
            pp.push(tmp);
            to.push((tmp, i));
        }
    }
    pp.sort_unstable(); pp.dedup();
    let mut ans = vec![(0, 0); pp.len()];
    for i in 0..pp.len() {
        ans[i].1 = pp[i];
    }
    for &(p, v) in &to {
        let idx = pp.binary_search(&p).unwrap();
        ans[idx].0 |= 1 << v;
    }
    ans
}

// O(n^3), n <= 128
fn build_rooted_tree_from_ancestor_relation(g: &[u128]) -> Vec<Vec<usize>> {
    let n = g.len();
    let mut ch = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if (g[i] & 1 << j) == 0 {
                continue;
            }
            let mut ok = true;
            for k in 0..n {
                if (g[i] & 1 << k) != 0 && (g[k] & 1 << j) != 0 {
                    ok = false;
                    break;
                }
            }
            if ok {
                ch[i].push(j);
            }
        }
    }
    ch
}

fn arrangable(comps: &[u128]) -> Option<Vec<u128>> {
    assert_ne!(comps.len(), 0); 
    if comps.len() == 1 {
        return Some(comps.to_vec());
    }
    let n = comps.len();
    let mut idx = n;
    for i in 0..n {
        let mut uf = UnionFind::new(n);
        for j in 0..n {
            if i != j {
                for k in 0..j {
                    if i != k {
                        let and = comps[j] & comps[k];
                        if and != 0 && and != comps[j] && and != comps[k] {
                            uf.unite(j, k);
                        }
                    }
                }
            }
        }
        if uf.size((i + 1) % n) == n - 1 {
            idx = i;
        }
    }
    let mut comps_sub = comps.to_vec();
    comps_sub.remove(idx);
    // since rustc 1.22.0
    let sub = arrangable(&comps_sub)?;
    let mut uni = 0;
    for i in 0..n - 1 {
        uni |= comps_sub[i];
    }
    let z0 = comps[idx] & !uni;
    let m = sub.len();
    let mut int = vec![];
    for i in 0..m {
        if (sub[i] & comps[idx]) != 0 && (sub[i] & !comps[idx]) != 0 {
            int.push(i);
        }
    }
    if int.len() >= 3 {
        return None;
    }
    let mut cnt = 0;
    let mut l = m;
    let mut r = 0;
    for i in 0..m {
        if (sub[i] & comps[idx]) != 0 {
            cnt += 1;
            r = max(r, i);
            l = min(l, i);
        }
    }
    if cnt + l != r + 1 {
        return None;
    }
    // divide int's elements by comps[idx]
    let mut div = vec![];
    for i in 0..m {
        let s = sub[i];
        if (s & comps[idx]) != 0 && (s & !comps[idx]) != 0 {
            if (m >= 2 && l == m - 1 && r == m - 1 && i == m - 1) || (i <= l && i < r) {
                div.push(s & !comps[idx]);
                div.push(s & comps[idx]);
            } else {
                div.push(s & comps[idx]);
                div.push(s & !comps[idx]);
            }
        } else {
            div.push(s);
        }
    }
    // Not in the tutorial, but it's necessary
    if int.is_empty() {
        if l == 0 || r == m - 1 {
            if z0 != 0 {
                if l == 0 {
                    div.insert(0, z0);
                } else {
                    div.push(z0);
                }
            }
            return Some(div);
        }
        if z0 != 0 {
            return None;
        }
        return Some(div);
    }
    if int.len() == 1 {
        let v = int[0];
        if (l == 0 && r == v) || (l == v && r == m - 1) {
            if z0 != 0 {
                if l == 0 && r == v {
                    div.insert(0, z0);
                } else {
                    div.push(z0);
                }
            }
            return Some(div)
        }
        if l == v || r == v {
            return if z0 != 0 {
                None
            } else {
                Some(div)
            };
        }
        return None;
    }
    assert_eq!(int.len(), 2);
    let v = int[0];
    let w = int[1];
    assert!(v < w);
    if l == v && r == w {
        if z0 != 0 {
            return None;
        }
        return Some(div);
    }
    None
}

fn dfs(v: usize, ch: &[Vec<usize>], ss: &[u128], uf: &mut UnionFind,
       uni: &[u128]) -> MInt {
    let m = ch.len();
    let mut prod = MInt::new(1);
    if uf.size(v) > 1 {
        prod = 2.into();
    }
    let mut comps = vec![];
    for i in 0..m {
        if uf.root(i) == v {
            comps.push(ss[i]);
        }
    }
    // Is it possible to arrange the elements of uni[i]?
    let sub = if let Some(sub) = arrangable(&comps) {
        sub
    } else {
        return 0.into();
    };
    let k = sub.len();
    // How many indivisibles comps[i] essentially has
    let mut card = vec![0; k];
    for i in 0..k {
        card[i] = sub[i].count_ones();
    }
    for &w in &ch[v] {
        prod *= dfs(w, ch, ss, uf, uni);
        for i in 0..k {
            if (sub[i] & uni[w]) == uni[w] {
                card[i] -= uni[w].count_ones() - 1;
            }
        }
    }
    for i in 0..k {
        for j in 1..card[i] + 1 {
            prod *= j as i64;
        }
    }
    prod
}

// The author solved this problem after reading the tutorial.
// Tags: making-rooted-trees, essentially-unique-enumeration, equivalence-classes, rooted-trees-from-ancestor-relations, hamiltonian-paths, permutations
fn main() {
    input! {
        n: usize, m: usize,
        s: [[usize1]; m],
    }
    let mut ss: Vec<u128> = vec![0; m];
    for i in 0..m {
        for &v in &s[i] {
            ss[i] |= 1 << v;
        }
    }
    ss.push((1 << n) - 1);
    let mut uf = UnionFind::new(m + 1);
    for i in 0..m + 1 {
        for j in 0..i {
            let int = ss[i] & ss[j];
            if ss[i] != int && ss[j] != int && int != 0 {
                uf.unite(i, j);
            }
        }
    }
    let mut uni = vec![0; m + 1];
    for i in 0..m + 1 {
        uni[uf.root(i)] |= ss[i];
    }
    let mut anc = vec![0; m + 1];
    for i in 0..m + 1 {
        if uni[i] == 0 {
            continue;
        }
        for j in 0..m + 1 {
            if uni[j] == 0 || i == j {
                continue;
            }
            if uni[j] == uni[i] {
                if uf.size(i) == 1 && (uf.size(j) != 1 || j < i) {
                    anc[i] |= 1 << j;
                }
            } else if (uni[i] & uni[j]) == uni[j] {
                anc[i] |= 1 << j;
            }
        }
    }
    let ch = build_rooted_tree_from_ancestor_relation(&anc);
    println!("{}", dfs(m, &ch, &ss, &mut uf, &uni));
}
