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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

/// https://judge.yosupo.jp/submission/5155
mod pollard_rho {
    use std::collections::HashMap;
    /// binary gcd
    pub fn gcd(mut x: i64, mut y: i64) -> i64 {
        if y == 0 { return x; }
        if x == 0 { return y; }
        let k = (x | y).trailing_zeros();
        y >>= k;
        x >>= x.trailing_zeros();
        while y != 0 {
            y >>= y.trailing_zeros();
            if x > y { let t = x; x = y; y = t; }
            y -= x;
        }
        x << k
    }

    fn add_mod(x: i64, y: i64, n: i64) -> i64 {
        let z = x + y;
        if z >= n { z - n } else { z }
    }

    fn mul_mod(x: i64, mut y: i64, n: i64) -> i64 {
        assert!(x >= 0);
        assert!(x < n);
        let mut sum = 0;
        let mut cur = x;
        while y > 0 {
            if (y & 1) == 1 {
                sum = add_mod(sum, cur, n);
            }
            cur = add_mod(cur, cur, n);
            y >>= 1;
        }
        sum
    }

    fn mod_pow(x: i64, mut e: i64, n: i64) -> i64 {
        let mut prod = if n == 1 { 0 } else { 1 };
        let mut cur = x % n;
        while e > 0 {
            if (e & 1) == 1 {
                prod = mul_mod(prod, cur, n);
            }
            e >>= 1;
            if e > 0 {
                cur = mul_mod(cur, cur, n);
            }
        }
        prod
    }

    pub fn is_prime(n: i64) -> bool {
        if n <= 1 { return false; }
        let small = [2, 3, 5, 7, 11, 13];
        if small.iter().any(|&u| u == n) { return true; }
        if small.iter().any(|&u| n % u == 0) { return false; }
        let mut d = n - 1;
        let e = d.trailing_zeros();
        d >>= e;
        // https://miller-rabin.appspot.com/
        let a = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
        a.iter().all(|&a| {
            if a % n == 0 { return true; }
            let mut x = mod_pow(a, d, n);
            if x == 1 { return true; }
            for _ in 0 .. e {
                if x == n - 1 {
                    return true;
                }
                x = mul_mod(x, x, n);
                if x == 1 { return false; }
            }
            x == 1
        })
    }

    fn pollard_rho(n: i64, c: &mut i64) -> i64 {
        // An improvement with Brent's cycle detection algorithm is performed.
        // https://maths-people.anu.edu.au/~brent/pub/pub051.html
        if n % 2 == 0 { return 2; }
        loop {
            let mut x: i64; // tortoise
            let mut y = 2; // hare
            let mut d = 1;
            let cc = *c;
            let f = |i| add_mod(mul_mod(i, i, n), cc, n);
            let mut r = 1;
            // We don't perform the gcd-once-in-a-while optimization
            // because the plain gcd-every-time algorithm appears to
            // outperform, at least on judge.yosupo.jp :)
            while d == 1 {
                x = y;
                for _ in 0..r {
                    y = f(y);
                    d = gcd((x - y).abs(), n);
                    if d != 1 {
                        break;
                    }
                }
                r *= 2;
            }
            if d == n {
                *c += 1;
                continue;
            }
            return d;
        }
    }

    /// Outputs (p, e) in p's ascending order.
    pub fn factorize(x: i64) -> Vec<(i64, usize)> {
        if x <= 1 {
            return Vec::new();
        }
        let mut hm = HashMap::new();
        let mut pool = vec![x];
        let mut c = 1;
        while let Some(u) = pool.pop() {
            if is_prime(u) {
                *hm.entry(u).or_insert(0) += 1;
                continue;
            }
            let p = pollard_rho(u, &mut c);
            pool.push(p);
            pool.push(u / p);
        }
        let mut v: Vec<_> = hm.into_iter().collect();
        v.sort();
        v
    }
} // mod pollard_rho

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

// Finds #GL(k, Z/(p^n)).
fn gl(p: ModInt, n: i64, k: i64) -> ModInt {
    let mut prod = ModInt::new(1);
    let pk = p.pow(k);
    for i in 0..k {
        prod *= pk - p.pow(i);
    }
    prod * p.pow((n - 1) * k * k)
}

fn dfs(p: i64, e: usize, last: usize, path: &mut Vec<(usize, usize)>) -> ModInt {
    if e == 0 {
        let p = ModInt::new(p);
        let mut prod = ModInt::new(1);
        let mut pexp: i64 = 0;
        for i in 0..path.len() {
            let (n, k) = path[i];
            prod *= gl(p, n as i64, k as i64);
            for j in 0..path.len() {
                if i == j { continue; }
                let exp = min(path[j].0, n) as i64
                    * path[j].1 as i64 * k as i64;
                pexp += exp;
            }
        }
        return (prod * p.pow(pexp)).inv();
    }
    if last == 0 {
        return 0.into();
    }
    let mut sum = ModInt::new(0);
    for i in 1..last + 1 {
        for j in 1..e / i + 1 {
            path.push((i, j));
            sum += dfs(p, e - i * j, i - 1, path);
            path.pop();
        }
    }
    sum
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        m: i64,
    }
    let facs = pollard_rho::factorize(m);
    let mut prod = ModInt::new(1);
    for i in 1..m + 1 {
        prod *= i;
    }
    for &(p, e) in &facs {
        prod *= dfs(p, e, e, &mut vec![]);
    }
    puts!("{}\n", prod);
}