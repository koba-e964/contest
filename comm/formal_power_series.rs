/// Computes f^{-1} mod x^{f.len()}.
///
/// Reference: https://codeforces.com/blog/entry/56422
///
/// Complexity: O(n log n)
///
/// Verified by: https://yukicoder.me/submissions/415313
///
/// Depends on: ModInt.rs, fft.rs
fn formal_power_series_inv<P: mod_int::Mod + PartialOrd>(
    f: &[mod_int::ModInt<P>],
    gen: mod_int::ModInt<P>
) -> Vec<mod_int::ModInt<P>> {
    let n = f.len();
    assert!(n.is_power_of_two());
    assert_eq!(f[0], 1.into());
    let mut sz = 1;
    let mut r = vec![mod_int::ModInt::new(0); n];
    r[0] = 1.into();
    while sz < n {
        sz *= 2;
        // r_{i + 1} = 2 * r_i - r_i^2 * f
        let zeta = gen.pow((P::m() - 1) / sz as i64 / 2);
        let mut tmp_r = vec![mod_int::ModInt::new(0); 2 * sz];
        let mut tmp_f = vec![mod_int::ModInt::new(0); 2 * sz];
        for i in 0..sz {
            tmp_r[i] = r[i];
            tmp_f[i] = f[i];
        }
        fft::transform(&mut tmp_r, zeta, 1.into());
        fft::transform(&mut tmp_f, zeta, 1.into());
        let fac = mod_int::ModInt::new(2 * sz as i64).inv();
        for i in 0..2 * sz {
            tmp_r[i] = tmp_r[i] * (-tmp_r[i] * tmp_f[i] + 2) * fac;
        }
        fft::transform(&mut tmp_r, zeta.inv(), 1.into());
        for i in 0..sz {
            r[i] = tmp_r[i];
        }
    }
    r
}
