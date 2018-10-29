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

const MOD: i64 = 1_000_000_007;
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

/// Generates an Iterator over subsets of univ, in the descending order. 
/// Verified by: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3050308
struct SubsetIter { bits: Option<usize>, univ: usize }
impl Iterator for SubsetIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self.bits {
            None => None,
            Some(bits) => {
                let ans = bits;
                self.bits =
                    if bits == 0 { None }
                else { Some((bits - 1) & self.univ) };
                Some(ans)
            }
        }
    }
}
fn subsets(univ: usize) -> SubsetIter {
    SubsetIter { bits: Some(univ), univ }
}


fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        s: [usize; n],
    }
    const W: usize = 17;
    let mut fib = vec![ModInt::new(0); 1 << W];
    fib[1] = ModInt::new(1);
    for i in 2 .. 1 << W {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    let mut freq = vec![ModInt::new(0); 1 << W];
    for &s in s.iter() {
        freq[s] += ModInt::new(1);
    }
    let mut v1 = vec![ModInt::new(0); 1 << W];
    for bits in 0 .. 1 << W {
        for sub in subsets(bits) {
            v1[bits] += freq[sub] * freq[bits - sub];
        }
    }
    // eprintln!("v1 = {:?}", &v1[0..10]);
    let mut v3 = freq.clone();
    for i in 0 .. W {
        for bits in 0 .. 1 << W {
            if (bits & 1 << i) == 0 {
                let nxt = bits ^ 1 << i;
                let x = v3[bits] + v3[nxt];
                let y = v3[bits] - v3[nxt];
                v3[bits] = x;
                v3[nxt] = y;
            }
        }
    }
    for bits in 0 .. 1 << W {
        v3[bits] *= v3[bits];
    }
    for i in 0 .. W {
        for bits in 0 .. 1 << W {
            if (bits & 1 << i) == 0 {
                let nxt = bits ^ 1 << i;
                let x = v3[bits] + v3[nxt];
                let y = v3[bits] - v3[nxt];
                v3[bits] = x;
                v3[nxt] = y;
            }
        }
    }
    let fac = ModInt::new(2).pow(MOD - 1 - W as i64);
    for bits in 0 .. 1 << W {
        v3[bits] *= fac;
    }
    // eprintln!("v3 = {:?}", &v3[..10]);
    let mut v2 = freq.clone();
    for bits in 0 .. 1 << W {
        v1[bits] *= fib[bits];
        v2[bits] *= fib[bits];
        v3[bits] *= fib[bits];
    }
    for i in 0 .. W {
        for bits in 0 .. 1 << W {
            if (bits & 1 << i) == 0 {
                let nxt = bits ^ 1 << i;
                v1[bits] += v1[nxt];
                v2[bits] += v2[nxt];
                v3[bits] += v3[nxt];
            }
        }
    }
    let mut ans = vec![ModInt::new(0); 1 << W];
    for bits in 0 .. 1 << W {
        ans[bits] = v1[bits] * v2[bits] * v3[bits];
    }
    for i in 0 .. W {
        for bits in 0 .. 1 << W {
            if (bits & 1 << i) == 0 {
                let nxt = bits ^ 1 << i;
                ans[bits] -= ans[nxt];
            }
        }
    }
    let mut tot = ModInt::new(0);
    for i in 0 .. W {
        tot += ans[1 << i];
    }
    puts!("{}\n", tot);
    
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
