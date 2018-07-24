#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }


/**
 * Sparse Table.
 * BiOp should be the type of a binary operator which is
 * associative, commutative and idempotent.
 * (For example, both min and gcd satisfy these properties.)
 * Verified by: AtCoder CODE FESTIVAL 2016 Tournament Round 3 (Parallel) B
 * (http://cf16-tournament-round3-open.contest.atcoder.jp/submissions/1026294)
 */
struct SparseTable<T, BiOp> {
    biop: BiOp,
    st: Vec<Vec<T>>,
}

impl<T, BiOp> SparseTable<T, BiOp>
    where BiOp: Fn(T, T) -> T,
          T: Copy {
    pub fn new(ary: &[T], biop: BiOp) -> Self {
        let n = ary.len();
        let mut h = 1;
        while 1 << h < n {
            h += 1;
        }
        let mut st: Vec<Vec<T>> = vec![Vec::from(ary); h + 1];
        for i in 0 .. n {
            st[0][i] = ary[i];
        }
        for b in 1 .. (h + 1) {
            if n + 1 < 1 << b {
                break;
            }
            for i in 0 .. (n + 1 - (1 << b)) {
                let next_idx = (1 << (b - 1)) + i;
                st[b][i] = biop(st[b - 1][i], st[b - 1][next_idx]);
            }
        }
        SparseTable {biop: biop, st: st}
    }
    fn top_bit(t: usize) -> usize {
        let mut h = 0;
        while 1 << h <= t {
            h += 1;
        }
        h - 1
    }
    pub fn query(&self, f: usize, s: usize) -> T {
        assert!(f <= s);
        let b = Self::top_bit(s + 1 - f);
        let endpoint = s + 1 - (1 << b);
        (self.biop)(self.st[b][f], self.st[b][endpoint])
    }
}

const MOD: i64 = 1_000_000_007;

/// Refers external ::MOD.
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
        pub fn new(x: i64) -> Self { ModInt { x: x % MOD } }
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
    impl ::std::fmt::Display for ModInt {
        fn fmt(&self, f: &mut::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
} // mod mod_int

fn get_period(p: &[usize]) -> (Vec<usize>, Vec<ModInt>) {
    let n = p.len();
    let mut ret = vec![0; n];
    let mut avg = vec![ModInt::new(0); n];
    for i in 0 .. n {
        if ret[i] != 0 { continue; }
        let mut cur = p[i];
        let mut count = 1;
        let mut tot = ModInt::new(i as i64);
        while cur != i {
            tot = tot + ModInt::new(cur as i64);
            cur = p[cur];
            count += 1;
        }
        tot = tot * ModInt::new(count as i64).inv();
        ret[i] = count;
        avg[i] = tot;
        cur = p[i];
        while cur != i {
            ret[cur] = count;
            avg[cur] = tot;
            cur = p[cur];
        }
        
    }
    (ret, avg)
}

use mod_int::*;

const W: usize = 41;

fn get_lcm(set: i64, precomp: &[Vec<(i32, i32)>]) -> ModInt {
    let mut ans = ModInt::new(1);
    let mut fac = HashMap::new();
    for i in 1 .. W {
        if (set & 1 << i) != 0 {
            for &(p, e) in &precomp[i] {
                let mut v = fac.entry(p).or_insert(0);
                *v = max(*v, e);
            }
        }
    }
    for (p, e) in fac {
        ans = ans * ModInt::new(p as i64).pow(e as i64);
    }
    ans
}

fn solve() {
    let mut precomp = vec![Vec::new(); W];
    for i in 1 .. W {
        let mut v = i as i32;
        let mut ans = Vec::new();
        let mut p = 2;
        while v > 1 {
            let mut e = 0;
            while v % p == 0 {
                v /= p;
                e += 1;
            }
            if e > 0 {
                ans.push((p, e));
            }
            p += 1;
        }
        precomp[i] = ans;
    }
    let n = get();
    let q = get();
    let p: Vec<_> = (0 .. n).map(|_| get::<usize>() - 1).collect();
    let (period, avg) = get_period(&p);
    let back: Vec<i64> = period.iter().map(|&p| 1 << p).collect();
    let spt = SparseTable::new(&back, |x, y| x | y);
    let mut avgacc = vec![ModInt::new(0); n + 1];
    for i in 0 .. n {
        avgacc[i + 1] = avgacc[i] + avg[i];
    }
    for _ in 0 .. q {
        let l = get::<usize>() - 1;
        let r: usize = get::<usize>() - 1;
        let set = spt.query(l, r);
        let lcm = get_lcm(set, &precomp);
        let range_sum = avgacc[r + 1] - avgacc[l];
        println!("{}", lcm * (range_sum + ModInt::new((r - l + 1) as i64)));
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
