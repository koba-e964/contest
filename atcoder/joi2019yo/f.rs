#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
const MOD: i64 = 10_007;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

fn fact(n: i32) -> MInt {
    let tbl = [1, 1, 2, 6, 24];
    tbl[n as usize].into()
}

fn invfact(n: i32) -> MInt {
    let tbl = [1, 1, (MOD + 1) / 2, (MOD + 1) / 6, (MOD + 1) / 24];
    tbl[n as usize].into()
}

fn dfs(a: i32, pos: usize, k: [i32; 5], del: [i32; 5],
       val: MInt, to: &mut Vec<([i32; 5], MInt)>) {
    let trans = [(2, 2), (3, 3), (3, 2), (3, 2), (2, 2)];
    let inert = [0, 4, 0, 4, 4];
    if pos == 5 {
        if a != 0 {
            return;
        }
        let mut del = del;
        for i in 0..5 {
            del[inert[i]] += k[i];
        }
        to.push((del, val));
        return;
    }
    dfs(a, pos + 1, k, del, val, to);
    for i in if pos == 4 { max(1, a) } else { 1 }..a + 1 {
        let fac = invfact(i);
        let (x, y) = trans[pos];
        // i = 1 + ... + 1
        if i <= k[pos] {
            let mut nk = k;
            let mut ndel = del;
            nk[pos] -= i;
            ndel[x] += i;
            ndel[y] += i;
            let mut cur = MInt::new(1);
            for j in 0..i {
                cur *= (k[pos] - j) as i64;
            }
            dfs(a - i, pos + 1, nk, ndel, val * cur * fac, to);
        }
        // i = i
        if i >= 2 && k[pos] >= 1 {
            let mut nk = k;
            let mut ndel = del;
            nk[pos] -= 1;
            ndel[x] += 1;
            ndel[y] += 1;
            ndel[1] += i - 1;
            dfs(a - i, pos + 1, nk, ndel,
                val * k[pos] as i64 * fact(i) * fac, to);
        }
        // 3 = 2 + 1, 4 = 3 + 1
        if i >= 3 && k[pos] >= 2 {
            let mut nk = k;
            let mut ndel = del;
            nk[pos] -= 2;
            ndel[x] += 2;
            ndel[y] += 2;
            ndel[1] += i - 2;
            dfs(a - i, pos + 1, nk, ndel,
                val * k[pos] as i64 * fact(i) * (k[pos] - 1) as i64 * fac, to);
        }
        if i == 4 {
            // TODO
            // 4 = 2 + 2
            if k[pos] >= 2 {
                let mut nk = k;
                let mut ndel = del;
                nk[pos] -= 2;
                ndel[x] += 2;
                ndel[y] += 2;
                ndel[1] += 2;
                dfs(a - i, pos + 1, nk, ndel,
                    val * k[pos] as i64 * 12 * (k[pos] - 1) as i64 * fac, to);
            }
            // 4 = 2 + 1 + 1
            if k[pos] >= 3 {
                let mut nk = k;
                let mut ndel = del;
                nk[pos] -= 3;
                ndel[x] += 3;
                ndel[y] += 3;
                ndel[1] += 1;
                dfs(a - i, pos + 1, nk, ndel,
                    val * k[pos] as i64 * 12 * (k[pos] - 1) as i64 * (k[pos] - 2) as i64 * fac, to);
            }
        }
    }
}

// Tags: insertion-dp, implementation, partition-number
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let init = [
        1, // free
        0, // [i-1] [i-1], banned
        0, // [i-1] [other]
        0, // [i-1] [i-2], banned
        0, // [other] [other], but banned
    ];
    let mut hm = HashMap::new();
    hm.insert(init, MInt::new(1));
    for a in a {
        let mut nxt = HashMap::new();
        eprintln!("|hm| = {}", hm.len());
        for (k, v) in hm {
            let mut to = vec![];
            dfs(a, 0, k, [0; 5], fact(a), &mut to);
            for (to, coef) in to {
                *nxt.entry(to).or_insert(MInt::new(0)) += v * coef;
            }
        }
        hm = nxt;
    }
    let mut tot = MInt::new(0);
    for (k, v) in hm {
        if k[1] == 0 && k[3] == 0 && k[4] == 0 {
            tot += v;
        }
    }
    println!("{}", tot);
}
