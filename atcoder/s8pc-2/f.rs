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

const MOD: i64 = 1_000_000_007;

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

/// Refers external ::MOD.
mod mod_int {
    use ::MOD;
    use std::ops::*;
    #[derive(Copy, Clone, Debug)]
    pub struct ModInt {
        pub x: i64,
    }
    impl ModInt {
        pub fn new(x: i64) -> Self { ModInt { x: x } }
        pub fn mul_fast(self, others: Self) -> Self {
            Self::new(self.x * others.x % MOD)
        }
    }
    impl Add for ModInt {
        type Output = Self;
        fn add(self, others: Self) -> Self {
            let mut sum = self.x + others.x;
            if sum >= MOD {
                sum -= MOD;
            }
            Self::new(sum)
        }
    }
    impl Sub for ModInt {
        type Output = Self;
        fn sub(self, others: Self) -> Self {
            let mut sum = self.x - others.x;
            if sum < 0 {
                sum += MOD;
            }
            Self::new(sum)
        }
    }
    impl Mul for ModInt {
        type Output = Self;
        fn mul(self, others: Self) -> Self {
            // Naive multiplication in order to avoid overflow
            let mut sum = 0;
            let mut cur = self.x;
            let mut e = others.x;
            if self.x < others.x {
                cur = others.x;
                e = self.x;
            }
            while e > 0 {
                if e % 2 == 1 {
                    sum += cur;
                    if sum >= MOD {
                        sum -= MOD;
                    }
                }
                cur *= 2;
                if cur >= MOD {
                    cur -= MOD;
                }
                e /= 2;
            }
            Self::new(sum)
        }
    }
    impl BitXor<i64> for ModInt {
        type Output = Self;
        fn bitxor(self, others: i64) -> Self {
            Self::new(::powmod(self.x, others, MOD))
        }
    }
} // mod mod_int


fn solve() {
    use mod_int::*;
    let a: i64 = get();
    let b: ModInt = ModInt::new(get());
    let c: i64 = get();
    let mut tot = ModInt::new(0);
    let mut cur_comb = ModInt::new(1);
    let mut cur_pow = b ^ (a - 1);
    let binv = b ^ (MOD - 2);
    for i in 0 .. a {
        tot = tot + cur_comb.mul_fast(cur_pow);
        cur_pow = cur_pow.mul_fast(binv);
        cur_comb = cur_comb.mul_fast(ModInt::new(c + i));
        cur_comb = cur_comb.mul_fast(ModInt::new(i + 1) ^ (MOD - 2));
    }
    println!("{}", tot.x);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
