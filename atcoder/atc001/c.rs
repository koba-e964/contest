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

/// Complex numbers.
/// Verified by: ATC001-C (http://atc001.contest.atcoder.jp/submissions/1175487)
mod complex {
    use std::ops::{Add, Sub, Mul};
    #[derive(Clone, Copy, Debug)]
    pub struct Complex<T = f64> {
        pub x: T,
        pub y: T,
    }
    
    impl<T> Complex<T> {
        pub fn new(x: T, y: T) -> Self { Complex { x: x, y: y } }
    }
    impl<T> Add for Complex<T>
        where T: Add<Output = T> {
        type Output = Self;
        fn add(self, other: Self) -> Self { 
            Self::new(self.x + other.x, self.y + other.y)
        }
    }
    impl<T> Sub for Complex<T>
        where T: Sub<Output = T> {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            Self::new(self.x - other.x, self.y - other.y)
        }
    }
    impl<T: Copy> Mul for Complex<T>
        where T: Add<Output = T> +
              Sub<Output = T> +
              Mul<Output = T> {
        type Output = Self;
        fn mul(self, other: Self) -> Self {
            Self::new(self.x * other.x - self.y * other.y,
                      self.x * other.y + self.y * other.x)
        }
    }
}


/// FFT (in-place)
/// R: Ring + Copy
/// Verified by: ATC001-C (http://atc001.contest.atcoder.jp/submissions/1175487)
mod fft {
    use std::ops::*;
    fn inplace_internal_fft<R>(
        f: &[R], output: &mut [R], ztbl: &[R], one: R,
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
        inplace_internal_fft(f, output, ztbl, one, x + 1,
                             fstart, 2 * fstep, n / 2, ostart);
        inplace_internal_fft(f, output, ztbl, one, x + 1,
			     fstart + fstep, 2 * fstep, n / 2, ostart + n / 2);
        let zeta = ztbl[x];
        let mut pzeta = one;
        for i in 0 .. n / 2 {
            let f0 = output[ostart + i];
            let f1 = output[ostart + i + n / 2];
            output[ostart + i] = f0 + pzeta * f1;
            output[ostart + i + n / 2] = f0 - pzeta * f1;
            pzeta = pzeta * zeta;
        }
    }
    /// n should be a power of 2. zeta is a primitive n-th root of unity.
    /// one is unity
    /// Note that the result should be multiplied by 1/sqrt(n).
    pub fn transform<R: ::std::fmt::Debug>(f: &[R], zeta: R, one: R) -> Vec<R>
        where R: Copy +
        Add<Output = R> +
        Sub<Output = R> +
        Mul<Output = R> {
        let n = f.len();
        assert!(n.is_power_of_two());
        let p = n.trailing_zeros() as usize;
        let mut ztbl = vec![zeta; p];
        for i in 1 .. p {
            ztbl[i] = ztbl[i - 1] * ztbl[i - 1];
        }
        let mut output = vec![zeta; n];
        inplace_internal_fft(&f, &mut output, &ztbl, one, 0, 0, 1, n, 0); 
        output
    }
}


#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn solve() {
    use complex::Complex;
    let orign = get();
    let mut a = vec![0.0; orign];
    let mut b = vec![0.0; orign];
    for i in 0 .. orign {
        a[i] = get();
        b[i] = get();
    }
    let n = 2 * orign.next_power_of_two();
    let mut ca = vec![Complex::new(0.0, 0.0); n];
    let mut cb = vec![Complex::new(0.0, 0.0); n];
    for i in 0 .. orign {
        ca[i] = Complex::new(a[i], 0.0);
        cb[i] = Complex::new(b[i], 0.0);
    }
    let pi = 3.14159265358979;
    let nf = n as f64;
    let theta = 2.0 * pi / nf;
    let zeta = Complex::new(theta.cos(), theta.sin());
    let inv_zeta = Complex::new(theta.cos(), -theta.sin());
    let one = Complex::new(1.0, 0.0);
    let mut ca = fft::transform(&ca, zeta, one);
    let cb = fft::transform(&cb, zeta, one);
    for i in 0 .. n {
        ca[i] = ca[i] * cb[i];
    }
    let ca = fft::transform(&ca, inv_zeta, one);
    println!("0");
    for i in 0 .. 2 * orign - 1 {
        println!("{}", (ca[i].x / nf + 0.5).floor() as i64);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
