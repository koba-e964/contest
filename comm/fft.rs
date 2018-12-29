/// FFT (in-place, verified as NTT only)
/// R: Ring + Copy
/// Verified by: https://codeforces.com/contest/1096/submission/47672373
mod fft {
    use std::ops::*;
    /// n should be a power of 2. zeta is a primitive n-th root of unity.
    /// one is unity
    /// Note that the result should be multiplied by 1/sqrt(n).
    pub fn transform<R>(f: &mut [R], zeta: R, one: R)
        where R: Copy +
        Add<Output = R> +
        Sub<Output = R> +
        Mul<Output = R> {
        let n = f.len();
        assert!(n.is_power_of_two());
        {
            let mut i = 0;
            for j in 1 .. n - 1 {
                let mut k = n >> 1;
                loop {
                    i ^= k;
                    if k <= i { break; }
                    k >>= 1;
                }
                if j < i { f.swap(i, j); }
            }
        }
        let mut zetapow = Vec::new();
        {
            let mut m = 1;
            let mut cur = zeta;
            while m < n {
                zetapow.push(cur);
                cur = cur * cur;
                m *= 2;
            }
        }
        let mut m = 1;
        while m < n {
            let base = zetapow.pop().unwrap();
            let mut r = 0;
            while r < n {
                let mut w = one;
                for s in r .. r + m {
                    let u = f[s];
                    let d = f[s + m] * w;
                    f[s] = u + d;
                    f[s + m] = u - d;
                    w = w * base;
                }
                r += 2 * m;
            }
            m *= 2;
        }
    }
}
