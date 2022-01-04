use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
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
} // complex

type Comp = complex::Complex<i64>;

// https://yukicoder.me/problems/no/675 (3)
// Tags: accumulation, noncommutative-group, noncommutative-monoid
fn main() {
    let n: usize = get();
    let px: i64 = get();
    let py: i64 = get();
    let mut dat = vec![];
    for _ in 0..n {
        let ty: i32 = get();
        let mut me = (Comp::new(1, 0), Comp::new(0, 0));
        if ty != 3 {
            let d: i64 = get();
            if ty == 1 {
                me.1.x = d;
            } else {
                me.1.y = d;
            }
        } else {
            me.0 = Comp::new(0, -1);
        }
        dat.push(me);
    }
    let mut acc = vec![(Comp::new(1, 0), Comp::new(0, 0)); n + 1];
    for i in (0..n).rev() {
        let (x, y) = acc[i + 1];
        let (z, w) = dat[i];
        acc[i] = (z * x, w * x + y);
    }
    for i in 0..n {
        let (x, y) = acc[i];
        let p = Comp::new(px, py) * x + y;
        println!("{} {}", p.x, p.y);
    }
}
