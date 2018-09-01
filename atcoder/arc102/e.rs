#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

const MOD: i64 = 998244353;
/// Refers to external ::MOD.
/// Verified by: https://beta.atcoder.jp/contests/arc099/submissions/2893648
mod mod_int {
    use ::MOD;
    use std::ops::*;
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ModInt { pub x: i64 }
    impl ModInt {
        fn check_integrity(self) {
            debug_assert!(self.x >= 0);
            debug_assert!(self.x < MOD);
        }
        // x >= 0
        pub fn new<T: Into<i64>>(x: T) -> Self { ModInt { x: x.into() % MOD } }
        #[allow(dead_code)]
        pub fn mul_fast(self, other: Self) -> Self {
            self.check_integrity();
            other.check_integrity();
            ModInt { x: self.x * other.x % MOD }
        }
        #[allow(dead_code)]
        pub fn mul_slow(self, other: Self) -> Self {
            // Naive multiplication in order to avoid overflow
            self.check_integrity();
            other.check_integrity();
            let mut sum = ModInt::new(0);
            let mut cur = self;
            let mut e = other.x;
            if self.x < other.x {
                cur = other;
                e = self.x;
            }
            while e > 0 {
                if e % 2 == 1 {
                    sum = sum + cur;
                }
                cur = cur + cur;
                e /= 2;
            }
            sum
        }
        #[allow(dead_code)]
        pub fn pow(self, mut e: i64) -> Self {
            self.check_integrity();
            debug_assert!(e >= 0);
            let mut sum = ModInt::new(1);
            let mut cur = ModInt::new(self.x);
            while e > 0 {
                if e % 2 != 0 {
                    sum = sum * cur;
                }
                cur = cur * cur;
                e /= 2;
            }
            sum
        }
        #[allow(dead_code)]
        pub fn inv(self) -> Self { self.pow(MOD - 2) }
    }
    impl Add for ModInt {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            self.check_integrity();
            other.check_integrity();
            let mut sum = self.x + other.x;
            if sum >= MOD { sum -= MOD; }
            ModInt { x: sum }
        }
    }
    impl Sub for ModInt {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            self.check_integrity();
            other.check_integrity();
            let mut sum = self.x - other.x;
            if sum < 0 { sum += MOD; }
            ModInt { x: sum }
        }
    }
    impl Mul for ModInt {
        type Output = Self;
        fn mul(self, other: Self) -> Self {
            self.mul_fast(other)
        }
    }
    impl AddAssign for ModInt {
        fn add_assign(&mut self, rhs: ModInt) { *self = *self + rhs; }
    }
    impl SubAssign for ModInt {
        fn sub_assign(&mut self, rhs: ModInt) { *self = *self - rhs; }
    }
    impl MulAssign for ModInt {
        fn mul_assign(&mut self, rhs: ModInt) { *self = *self * rhs; }
    }
    impl ::std::fmt::Display for ModInt {
        fn fmt(&self, f: &mut::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
} // mod mod_int
use mod_int::*;


fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input!{
        k: usize,
        n: usize,
    }
    let mut dp = vec![ModInt::new(0); n + 1];
    dp[0] = ModInt::new(1);
    dp[1] = ModInt::new(1);
    for _ in 0 .. k - 1 {
        for j in 0 .. n {
            dp[j + 1] += dp[j];
        }
    }
    let mut ans = vec![ModInt::new(0); 2 * k + 1];
    for pl in 2 .. k + 2 {
        ans[pl] = dp[n];
        if pl % 2 == 1 {
            for j in (0 .. n).rev() {
                dp[j + 1] += dp[j];
            }
            for j in (0 .. n).rev() {
                dp[j + 1] -= dp[j];
            }
        }
    }
    for pl in k + 2 .. 2 * k + 1 {
        ans[pl] = ans[2 * k + 2 - pl];
    }
    for i in 2 .. 2 * k + 1 {
        puts!("{}\n", ans[i]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
