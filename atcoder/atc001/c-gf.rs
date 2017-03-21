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


/// FFT (in-place)
/// R: Ring + Copy
/// Verified by: ATC001-C (http://atc001.contest.atcoder.jp/submissions/1175827)
mod fft {
    use std::ops::*;
    fn inplace_internal_fft<R>(
        f: &[R], output: &mut [R], pztbl: &[R], one: R,
        x: usize, fstart: usize, fstep: usize,
        n: usize, ostart: usize)
        where R: Copy +
        Add<Output = R> +
        Sub<Output = R> +
        Mul<Output = R> {
        if n == 1 {
            output[ostart] = f[fstart];
            return;
        }
        inplace_internal_fft(f, output, pztbl, one, x + 1,
                             fstart, 2 * fstep, n / 2, ostart);
        inplace_internal_fft(f, output, pztbl, one, x + 1,
			     fstart + fstep, 2 * fstep, n / 2, ostart + n / 2);
        let mut cnt = 0;
        for i in 0 .. n / 2 {
            let pzeta = pztbl[cnt];
            let f0 = output[ostart + i];
            let f1 = output[ostart + i + n / 2];
            let tmp = pzeta * f1;
            output[ostart + i] = f0 + tmp;
            output[ostart + i + n / 2] = f0 - tmp;
            cnt += 1 << x;
        }
    }
    /// n should be a power of 2. zeta is a primitive n-th root of unity.
    /// one is unity
    /// Note that the result should be multiplied by 1/sqrt(n).
    pub fn transform<R>(f: &[R], zeta: R, one: R) -> Vec<R>
        where R: Copy +
        Add<Output = R> +
        Sub<Output = R> +
        Mul<Output = R> {
        let n = f.len();
        assert!(n.is_power_of_two());
        let mut pztbl = vec![one; n];
        for i in 1 .. n {
            pztbl[i] = pztbl[i - 1] * zeta;
        }
        let mut output = vec![zeta; n];
        inplace_internal_fft(&f, &mut output, &pztbl, one, 0, 0, 1, n, 0);
        output
    }
}


#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

/// 5 * 2^55 + 1 is a prime.
/// Source: http://mathworld.wolfram.com/ProthPrime.html
const MOD: i64 = (5 << 55) + 1;
/// a primitive root of 5 * 2^55 + 1
const PRIM: i64 = 11;

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
} // mod mod_int

use mod_int::*;
fn powmod(x: ModInt, mut e: i64) -> ModInt {
    use mod_int::*;
    let mut sum = ModInt::new(1);
    let mut cur = x;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur;
        }
        cur = cur * cur;
        e /= 2;
    }
    sum
}


fn solve() {
    let orign = get();
    let mut a = vec![0; orign];
    let mut b = vec![0; orign];
    for i in 0 .. orign {
        a[i] = get();
        b[i] = get();
    }
    let n = 2 * orign.next_power_of_two();
    let mut ca = vec![ModInt::new(0); n];
    let mut cb = vec![ModInt::new(0); n];
    for i in 0 .. orign {
        ca[i] = ModInt::new(a[i]);
        cb[i] = ModInt::new(b[i]);
    }
    let zeta = powmod(ModInt::new(PRIM), (MOD - 1) / n as i64);
    let inv_zeta = powmod(ModInt::new(PRIM), MOD - 1 - (MOD - 1) / n as i64);
    let one = ModInt::new(1);
    let mut ca = fft::transform(&ca, zeta, one);
    let cb = fft::transform(&cb, zeta, one);
    for i in 0 .. n {
        ca[i] = ca[i] * cb[i];
    }
    let ca = fft::transform(&ca, inv_zeta, one);
    println!("0");
    for i in 0 .. 2 * orign - 1 {
        println!("{}", ca[i].x / n as i64);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
