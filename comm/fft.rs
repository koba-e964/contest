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
