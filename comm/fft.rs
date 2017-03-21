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
