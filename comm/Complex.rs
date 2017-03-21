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
