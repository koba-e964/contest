/// Depends on: fft.rs, ModInt.rs
/// Primitive root defaults to 3 (for 998244353); for other moduli change the value of it.
fn conv(a: Vec<ModInt>, b: Vec<ModInt>) -> Vec<ModInt> {
    let n = a.len() - 1;
    let m = b.len() - 1;
    let mut p = 1;
    while p <= n + m {
        p *= 2;
    }
    let mut f = vec![ModInt::new(0); p];
    let mut g = vec![ModInt::new(0); p];
    for i in 0..n + 1 {
        f[i] = a[i];
    }
    for i in 0..m + 1 {
        g[i] = b[i];
    }
    let fac = ModInt::new(p as i64).inv();
    let zeta = ModInt::new(3).pow((MOD - 1) / p as i64);
    fft::transform(&mut f, zeta, 1.into());
    fft::transform(&mut g, zeta, 1.into());
    for i in 0..p {
        f[i] *= g[i] * fac;
    }
    fft::transform(&mut f, zeta.inv(), 1.into());
    f[..n + m + 1].to_vec()
}
