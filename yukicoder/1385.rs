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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

/// Complex numbers.
/// Verified by: ATC001-C (http://atc001.contest.atcoder.jp/submissions/1175487)
mod complex {
    use std::ops::{Add, Sub, Mul, Neg};
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
    impl<T: Copy + Neg<Output = T>> Complex<T> {
        pub fn conj(self) -> Self {
            Self::new(self.x, -self.y)
        }
    }
} // complex

// https://yukicoder.me/problems/no/1385 (3)
// 三角形の面積は 点の座標に関する線形関数 + 定数 として表せるので、累積和ができる。
// 具体的には、複素数 a, b, c に対してこれらを頂点とする三角形の面積は |Im((b-c)conj(a-c))|/2 である。
// 絶対値の中身は a, b, c が反時計回りに並んでいる時に正。
// 点列を時計回りに a[0], ..., a[N-1] として、j < k < i のときの a = a[j], b = a[k], c = a[i] としたときの和を計算することにする。
// p[i] = \sum_{j < k < i} conj(a[j])a[k] が計算できていれば、c = a[i] のときの Im 内部の和は
// (p[i] - conj(a[i]) \sum_{j < i} ja[j] - a[i] \sum_{j<i}(i - j - 1)conj(a[j]) + |a|^2 i(i-1)/2) / 2 である。
// これの計算のためには q[i] = \sum_{j < i} ja[j] と r[i] = \sum_{j < i} a[j] がわかっていればよく、
// これらを使うと (p[i] - conj(a[i]) q[i] + a[i]conj(q[j]) - a[i](i-1)conj(r[i]) + |a|^2 i(i-1)/2) / 2 である。
fn main() {
    input! {
        n: usize, l: f64,
        t: [f64; n],
    }
    use complex::*;
    let mut a = vec![Complex::new(0.0, 0.0); n];
    for i in 0..n {
        let angle = std::f64::consts::PI * t[i] * 2.0 / l;
        a[i] = Complex::new(angle.cos(), angle.sin());
    }
    let mut p = vec![Complex::new(0.0, 0.0); n + 1];
    let mut q = vec![Complex::new(0.0, 0.0); n + 1];
    let mut r = vec![Complex::new(0.0, 0.0); n + 1];
    for i in 0..n {
        r[i + 1] = r[i] + a[i];
        q[i + 1] = q[i] + a[i] * Complex::new(i as f64, 0.0);
        if i > 0 {
            p[i + 1] = p[i] + a[i] * r[i].conj();
        }
    }
    let mut ans = Complex::new(0.0, 0.0);
    for i in 0..n {
        ans = ans + p[i] - a[i].conj() * q[i] + a[i] * q[i].conj()
            - a[i] * r[i].conj() * Complex::new(i as f64 - 1.0, 0.0);
    }
    let nn = n as f64;
    println!("{}", ans.y * 3.0 / nn / (nn - 1.0) / (nn - 2.0));
}
