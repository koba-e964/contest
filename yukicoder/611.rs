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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
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
const MOD: i64 = 201_712_111;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

fn flip(a: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let h = a.len();
    let w = a[0].len();
    let mut ans = vec![vec!['-'; h]; w];
    for i in 0..h {
        for j in 0..w {
            ans[j][i] = a[i][j];
        }
    }
    ans
}

fn calc(a: &[Vec<char>]) -> (i32, MInt) {
    let h = a.len();
    let w = a[0].len();
    let mut dist = vec![vec![0; w]; h];
    let mut q = vec![vec![false; w]; h];
    let mut b = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '?' {
                q[i][j] = true;
                b[i][j] = 1;
            } else {
                b[i][j] = (a[i][j] as u8 - b'0') as i32;
            }
        }
    }
    const INF: i32 = 1 << 29;
    dist[0][0] = b[0][0];
    for i in 0..h {
        for j in 0..w {
            if i + j == 0 { continue; }
            let mut me = INF;
            if i > 0 {
                me = min(me, dist[i - 1][j]);
            }
            if j > 0 {
                me = min(me, dist[i][j - 1]);
            }
            dist[i][j] = me + b[i][j];
        }
    }
    let mut dp = vec![MInt::new(0); 1 << w];
    let mut ep = vec![MInt::new(0); 1 << w];
    dp[1] += 1;
    for i in 0..h {
        for j in 0..w {
            for v in &mut ep { *v = 0.into(); }
            if i + j == 0 { continue; }
            for bits in 0..1 << w {
                if dp[bits].x == 0 { continue; }
                let val = dp[bits];
                let up = if i > 0 { dist[i - 1][j] } else { INF };
                let left = if j > 0 { dist[i][j - 1] } else { INF };
                let mi = min(up, left);
                let is_mi = (up == mi && (bits & 1 << j) != 0) || (left == mi && (bits & 1 << (j - 1)) != 0);
                let nxt = (bits & !(1 << j)) | if is_mi {
                    1 << j
                } else {
                    0
                };
                ep[nxt] += val;
                if q[i][j] {
                    ep[bits & !(1 << j)] += val * 8;
                }
            }
            std::mem::swap(&mut dp, &mut ep);
        }
    }
    let mut ans = MInt::new(0);
    for bits in 1 << (w - 1)..1 << w {
        ans += dp[bits];
    }
    (dist[h - 1][w - 1], ans)
}

// https://yukicoder.me/problems/no/611 (4.5)
// HW <= 302
// min(H, W) <= 17 なので道幅 bitDP。
// どこが最短になり得るかを行ごとに添字で管理。
// Tags: pathwidth, exploit-small-constraints
fn main() {
    input! {
        h: usize, w: usize,
        a: [chars; h],
    }
    let a = if h < w {
        flip(a)
    } else {
        a
    };
    let (res, cnt) = calc(&a);
    println!("{}\n{}", res, cnt);
}
