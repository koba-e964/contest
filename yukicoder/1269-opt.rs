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
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

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
type MInt = mod_int::ModInt<P>;

// The Aho-Corasick automaton construction.
// Complexity: \sum |pat| * alpha
fn aho_corasick(pat: &[Vec<usize>], alpha: usize)
                        -> (Vec<Vec<usize>>, Vec<bool>) {
    let mut st = vec![vec![usize::MAX; alpha]];
    let mut fin = vec![false];
    let mut back = vec![0];
    for p in pat {
        let mut cur = 0;
        for i in 0..p.len() {
            let c = p[i];
            if st[cur][c] == usize::MAX {
                st.push(vec![usize::MAX; alpha]);
                fin.push(false);
                back.push(usize::MAX);
                st[cur][c] = st.len() - 1;
            }
            cur = st[cur][c];
        }
        fin[cur] = true;
    }
    // fill in back links
    // Note: states are *not necessarily* topologically sorted!
    // Therefore, we need to use a queue.
    let mut que = std::collections::VecDeque::new();
    que.push_back(0);
    while let Some(i) = que.pop_front() {
        assert_ne!(back[i], usize::MAX);
        if fin[back[i]] {
            fin[i] = true;
        }
        for j in 0..alpha {
            if st[i][j] != usize::MAX {
                let nxt = st[i][j];
                que.push_back(nxt);
                if i == 0 {
                    back[nxt] = 0;
                } else {
                    let mut cur = back[i];
                    while st[cur][j] == usize::MAX && cur > 0 {
                        assert_ne!(back[cur], usize::MAX);
                        cur = back[cur];
                    }
                    back[nxt] = [0, st[cur][j]][usize::from(st[cur][j] != usize::MAX)];
                }
            } 
        }
    }
    // fill in vacant transitions
    for i in 0..st.len() {
        for j in 0..alpha {
            if st[i][j] == usize::MAX {
                let mut cur = back[i];
                while st[cur][j] == usize::MAX && cur > 0 {
                    cur = back[cur];
                }
                st[i][j] = [0, st[cur][j]][usize::from(st[cur][j] != usize::MAX)];
            }
        }
    }
    (st, fin)
}

// Tags: aho-corasick, dp
fn main() {
    input! {
        n: usize, l: i64, r: i64,
    }
    let mut fibs = vec![];
    {
        let mut a = 1;
        let mut b = 2;
        while a <= r {
            if l <= a {
                fibs.push(a.to_string().bytes().map(|x| (x - b'0') as usize)
                          .collect::<Vec<_>>());
            }
            let c = a + b;
            a = b;
            b = c;
        }
    }
    eprintln!("fib = {:?}", fibs);
    // build an automaton
    let (trans, fin) = aho_corasick(&fibs, 10);
    let m = trans.len();
    let mut dp = vec![vec![[MInt::new(0); 2]; m]; n + 1];
    dp[0][0][0] += 1;
    for i in 0..n {
        for k in 0..m {
            let val0 = dp[i][k][0];
            let val1 = dp[i][k][1];
            for j in 0..10 {
                let to = trans[k][j];
                if fin[to] {
                    dp[i + 1][to][1] += val0 + val1;
                } else {
                    dp[i + 1][to][0] += val0;
                    dp[i + 1][to][1] += val1;
                }
            }
        }
    }
    let mut tot = MInt::new(0);
    for i in 0..m {
        tot += dp[n][i][0];
    }
    println!("{}", tot - 1); // remove 0
}
