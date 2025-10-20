fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
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

// Segment Tree. This data structure is useful for fast folding on intervals of an array
// whose elements are elements of monoid I. Note that constructing this tree requires the identity
// element of I and the operation of I.
// Verified by: yukicoder No. 2220 (https://yukicoder.me/submissions/841554)
struct SegTree<I, BiOp> {
    n: usize,
    orign: usize,
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
        SegTree {n: n, orign: n_, dat: vec![e; 2 * n - 1], op: op, e: e}
    }
    // ary[k] <- v
    pub fn update(&mut self, idx: usize, v: I) {
        debug_assert!(idx < self.orign);
        let mut k = idx + self.n - 1;
        self.dat[k] = v;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.op)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
        }
    }
    // [a, b) (half-inclusive)
    // http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/
    #[allow(unused)]
    pub fn query(&self, rng: std::ops::Range<usize>) -> I {
        let (mut a, mut b) = (rng.start, rng.end);
        debug_assert!(a <= b);
        debug_assert!(b <= self.orign);
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
}

fn simple(s: &[char], lst: MInt) -> MInt {
    let mut ans = MInt::new(0);
    let mut cur = MInt::new(0);
    let mut pw = MInt::new(1);
    for i in (0..s.len()).rev() {
        cur += pw * (s[i] as u8 - b'0') as i64;
        ans += cur * lst;
        pw *= 10;
    }
    let mut pwsum = MInt::new(1);
    for i in (0..s.len()).rev() {
        ans += pwsum * (s[i] as u8 - b'0') as i64 * (i + 1) as i64;
        pwsum = pwsum * 10 + 1;
    }
    ans
}

fn term(s: &[char], fst: i64, lst: i64) -> MInt {
    let mut ans = MInt::new(0);
    let mut lacc = vec![];
    let mut terms = vec![];
    // s[..?]
    {
        let mut stack = MInt::new(1);
        let mut cur = MInt::new(0);
        let mut cum = MInt::new(0);
        for i in 0..s.len() {
            if s[i] == '*' {
                stack *= cur;
                terms.push(cur);
                lacc.push(cum);
                cur = MInt::new(0);
                cum = MInt::new(0);
            } else {
                cur = cur * 10 + (s[i] as u8 - b'0') as i64;
                ans += stack * cur * fst;
                cum += cur;
            }
        }
        ans += stack * cur * fst * lst;
        terms.push(cur);
        lacc.push(cum);
    }
    // s[?..]
    {
        let mut stack = MInt::new(1);
        let mut cur = MInt::new(0);
        let mut pw = MInt::new(1);
        for i in (0..s.len()).rev() {
            if s[i] == '*' {
                stack *= cur;
                cur = MInt::new(0);
                pw = MInt::new(1);
            } else {
                cur = cur + pw * (s[i] as u8 - b'0') as i64;
                pw *= 10;
                ans += stack * cur * lst;
            }
        }
    }
    let mut mul = vec![0];
    for i in 0..s.len() {
        if s[i] == '*' {
            mul.push(i + 1);
        }
    }
    mul.push(s.len() + 1);
    let mut st = SegTree::new(terms.len(), |(x, a), (y, b)| {
        (x * y, x * b + a)
    }, (MInt::new(1), MInt::new(0)));
    for i in 0..terms.len() {
        st.update(i, (terms[i], lacc[i]));
    }
    let m = mul.len();
    for i in 0..m - 1 {
        ans += simple(&s[mul[i]..mul[i + 1] - 1], st.query(i + 1..m - 1).1);
    }
    ans
}

fn main() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut plus = vec![];
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        if s[i] == '+' {
            plus.push(i);
        }
        if s[i] != '+' && s[i] != '*' {
            acc[i + 1] = acc[i] + 1;
        } else {
            acc[i + 1] = acc[i];
        }
    }
    let mut ans = MInt::new(0);
    if plus.is_empty() {
        ans += term(&s, 0, 0);
    } else {
        let m = plus.len();
        ans += term(&s[..plus[0]], 0, acc[n] - acc[plus[0]]);
        for i in 0..m - 1 {
            ans += term(&s[plus[i] + 1..plus[i + 1]], acc[plus[i] + 1], acc[n] - acc[plus[i + 1]]);
        }
        ans += term(&s[plus[m - 1] + 1..], acc[plus[m - 1] + 1], 0);
    }
    println!("{ans}");
    if n <= 100 {
        let mut debug = MInt::new(0);
        for i in 0..n {
            if s[i] == '+' || s[i] == '*' {
                continue;
            }
            for j in i..n {
                if s[j] == '+' || s[j] == '*' {
                    continue;
                }
                let mut ans = MInt::new(0);
                let mut cur = MInt::new(0);
                let mut stack = MInt::new(1);
                for &c in &s[i..=j] {
                    if c == '*' {
                        stack *= cur;
                        cur = MInt::new(0);
                        continue;
                    }
                    if c == '+' {
                        ans += stack * cur;
                        stack = 1.into();
                        cur = 0.into();
                        continue;
                    }
                    let c = (c as u8 - b'0') as i64;
                    cur = cur * 10 + c;
                }
                ans += stack * cur;
                debug += ans;
            }
        }
        eprintln!("debug = {debug}");
    }
}
