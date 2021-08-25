// Lagrange inversion. Finds [x^k] f(g^{-1}(x)).
// Depends on: MInt.rs, fact_init.rs, fft.rs, fps/fps_inv.rs, fps/fps_exp.rs
fn fps_lagrange<P: mod_int::Mod + PartialEq>(
    k: usize,
    f: &[mod_int::ModInt<P>],
    g: &[mod_int::ModInt<P>],
    gen: mod_int::ModInt<P>,
    fac: &[mod_int::ModInt<P>],
    invfac: &[mod_int::ModInt<P>],
) -> mod_int::ModInt<P> {
    assert!(g[0] == 0.into());
    assert!(g[1] != 0.into());
    if k == 0 {
        return f[0];
    }
    let mut p = 1;
    while k >= p {
        p *= 2;
    }
    let mut ff = vec![mod_int::ModInt::new(0); p];
    let l = std::cmp::min(p, f.len());
    ff[..l].copy_from_slice(&f[..l]);
    let mut gg = vec![mod_int::ModInt::new(0); p];
    let l = std::cmp::min(p, g.len());
    gg[..l - 1].copy_from_slice(&g[1..l]);
    let gg0 = gg[0];
    let gg0inv = gg0.inv();
    for i in 0..l - 1 {
        gg[i] *= gg0inv;
    }
    let mut lngg = fps_ln(&gg, gen, fac, invfac);
    for i in 0..p {
        lngg[i] *= -mod_int::ModInt::new(k as i64);
    }
    let egg = fps_exp(&lngg, gen, fac, invfac);
    let mut tot = mod_int::ModInt::new(0);
    for i in 0..k {
        tot += ff[i + 1] * (i + 1) as i64 * egg[k - 1 - i];
    }
    tot * mod_int::ModInt::new(k as i64).inv() * gg0inv.pow(k as i64)
}
