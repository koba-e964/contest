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
        pub struct $struct_name {}
        impl mod_int::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

// \sum_{1 = i <= n, i != a, i != b} |i - a|
fn pat0(n: i64, a: i64, b: i64) -> i64 {
    if a > b {
        return pat0(n, n - a + 1, n - b + 1);
    }
    let mut tot = a * (a - 1) / 2;
    tot += (n - a) * (n - a + 1) / 2;
    tot -= b - a;
    tot
}

// \sum_{1 <= i <= n, 1 <= j <= n, i != j, i, j != a, b} |i - j|
fn pat1(n: i64, a: i64, b: i64) -> i64 {
    let mut tot = n * (n - 1) * (n + 1) / 3; 
    tot -= pat0(n, a, b) * 2;
    tot -= pat0(n, b, a) * 2;
    tot -= (a - b).abs() * 2;
    tot
}

struct GroupingDP<T> {
    map: std::collections::HashMap<T, usize>,
    grp: Vec<usize>,
    grps: Vec<Vec<usize>>,
    mult: Vec<MInt>,
    mat: Vec<Vec<MInt>>,
}

impl<T: std::hash::Hash + Eq + Clone> GroupingDP<T> {
    pub fn new(states: impl IntoIterator<Item = Vec<T>>) -> Self {
        let states_: Vec<_> = states.into_iter().collect();
        let mut map_ = std::collections::HashMap::new();
        let mut idx = 0;
        for i in 0..states_.len() {
            for state in &states_[i] {
                map_.insert(state.clone(), idx);
                idx += 1;
            }
        }
        let mut grp_ = vec![0; idx];
        let mut grps_ = vec![vec![]; states_.len()];
        idx = 0;
        for i in 0..states_.len() {
            for _ in &states_[i] {
                grp_[idx] = i;
                grps_[i].push(idx);
                idx += 1;
            }
        }
        let n = map_.len();
        Self {
            map: map_,
            grp: grp_,
            grps: grps_,
            mult: vec![MInt::new(0); n],
            mat: vec![vec![MInt::new(0); n]; n],
        }
    }
    pub fn index(&self, state: T) -> Option<usize> {
        self.map.get(&state).copied()
    }
    pub fn multiplicity(&mut self, state: T, val: MInt) {
        let idx = self.map[&state];
        self.mult[idx] = val;
    }
    pub fn trans(&mut self, from: T, to: T, val: MInt, agg: bool) {
        let idx1 = self.map[&from];
        let idx2 = self.map[&to];
        if agg {
            self.mat[idx1][idx2] = val * self.mult[idx1] * self.mult[idx2].inv();
        } else {
            self.mat[idx1][idx2] = val;
        }
    }
    pub fn trans_add(&mut self, from: T, to: T, val: MInt, agg: bool) {
        let idx1 = self.map[&from];
        let idx2 = self.map[&to];
        if agg {
            self.mat[idx1][idx2] += val * self.mult[idx1] * self.mult[idx2].inv();
        } else {
            self.mat[idx1][idx2] += val;
        }
    }
    pub fn to_mat(&self) -> Vec<Vec<MInt>> {
        // first, verify the transitions given are valid
        let n = self.map.len();
        // TODO
        let m = self.grps.len();
        let mut mat = vec![vec![MInt::new(0); m]; m];
        for i in 0..m {
            let ii = self.grps[i][0];
            for j in 0..n {
                mat[i][self.grp[j]] += self.mat[ii][j];
            }
        }
        mat
    }
}

// Tags: dp, linear-transformations, grouping-of-states, expected-value, combinatorics, sum-of-absolute-differences
fn main() {
    input! {
        n: usize, m: usize,
        p: [usize; n],
    }
    let nn = n as i64;
    let invn = MInt::new(nn).inv();
    let invnn1 = invn * MInt::new(nn - 1).inv();
    let mut dp = GroupingDP::new(vec![vec![(0, 1), (1, 0)], vec![(0, 2), (2, 0)], vec![(1, 2), (2, 1)], vec![(2, 2)]]);
    dp.multiplicity((0, 1), 1.into());
    dp.multiplicity((1, 0), 1.into());
    dp.multiplicity((0, 2), (nn - 2).into());
    dp.multiplicity((1, 2), (nn - 2).into());
    dp.multiplicity((2, 0), (nn - 2).into());
    dp.multiplicity((2, 1), (nn - 2).into());
    dp.multiplicity((2, 2), ((nn - 2) * (nn - 3)).into());
    // i <-> i + 1
    for j in 0..3 {
        for k in 0..3 {
            if dp.index((j, k)).is_none() {
                continue;
            }
            dp.trans((j, k), (j, k), invnn1 * (nn - 2) * (nn - 3), false);
            dp.trans_add((j, k), (k, j), invnn1 * 2, false);
        }
    }
    // i, i + 1 <-> other
    let coef = invnn1 * 2;
    // 0 -> 2, 1 -> 2
    for j in 0..3 {
        if j != 0 {
            dp.trans((0, j), (2, j), coef, false);
            dp.trans((j, 0), (j, 2), coef, false);
        }
        if j != 1 {
            dp.trans((1, j), (2, j), coef, false);
            dp.trans((j, 1), (j, 2), coef, false);
        }
    }
    // 0 -> 1, 1 -> 0
    for i in 0..2 {
        dp.trans((i, 2), (1 - i, 2), coef, false);
        dp.trans((2, i), (2, 1 - i), coef, false);
    }
    // 2 -> 2
    for i in 0..2 {
        dp.trans_add((i, 2), (i, 2), coef * (nn - 3), false);
        dp.trans_add((2, i), (2, i), coef * (nn - 3), false);
    }
    dp.trans_add((2, 2), (2, 2), coef * (nn - 4) * 2, false);
    // 2 -> 0, 2 -> 1
    for j in 0..3 {
        for i in 0..2 {
            if i != j {
                dp.trans((2, j), (i, j), coef, true);
                dp.trans((j, 2), (j, i), coef, true);
            }
        }
    }
    let trans = dp.to_mat();

    let k = trans.len();
    let mut dist = vec![MInt::new(0); k];
    dist[dp.index((0, 1)).unwrap()] += 1;
    for _ in 0..m {
        let mut new = vec![MInt::new(0); k];
        for i in 0..k {
            for j in 0..k {
                new[j] += dist[i] * trans[i][j];
            }
        }
        dist = new;
    }
    let mut tot = MInt::new(0);
    for i in 0..n - 1 {
        let d = (p[i + 1] as i64 - p[i] as i64).abs();
        tot += dist[0] * d;
        tot += dist[1] * pat0(nn, p[i] as i64, p[i + 1] as i64);
        tot += dist[2] * pat0(nn, p[i + 1] as i64, p[i] as i64);
        tot += dist[3] * pat1(nn, p[i] as i64, p[i + 1] as i64);
    }
    tot *= MInt::new((nn * nn - nn) / 2).pow(m as i64);
    println!("{}", tot);
}
