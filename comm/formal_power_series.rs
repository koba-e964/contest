/// Computes f^{-1} mod x^{f.len()}.
///
/// Reference: https://codeforces.com/blog/entry/56422
///
/// Complexity: O(n log n)
///
/// Verified by: https://judge.yosupo.jp/submission/3219
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
    let mut r = vec![mod_int::ModInt::new(0); 2 * n];
    let mut tmp_f = vec![mod_int::ModInt::new(0); 2 * n];
    r[0] = 1.into();
    while sz < n {
        sz *= 2;
        // r_{i + 1} = 2 * r_i - r_i^2 * f
        let zeta = gen.pow((P::m() - 1) / sz as i64 / 2);
        for i in 0..sz {
            tmp_f[i] = f[i];
        }
        fft::transform(&mut r[..2 * sz], zeta, 1.into());
        fft::transform(&mut tmp_f[..2 * sz], zeta, 1.into());
        let fac = mod_int::ModInt::new(2 * sz as i64).inv();
        for i in 0..2 * sz {
            r[i] = r[i] * (-r[i] * tmp_f[i] + 2) * fac;
        }
        fft::transform(&mut r[..2 * sz], zeta.inv(), 1.into());
        for i in sz..2 * sz {
            r[i] = 0.into();
        }
    }
    r.truncate(sz);
    r
}
