// Depends on: fft.rs, MInt.rs
// Verified by: ABC269-Ex (https://atcoder.jp/contests/abc269/submissions/39116328)
pub struct FPSOps<M: mod_int::Mod> {
    gen: mod_int::ModInt<M>,
}

impl<M: mod_int::Mod> FPSOps<M> {
    pub fn new(gen: mod_int::ModInt<M>) -> Self {
        FPSOps { gen: gen }
    }
}

impl<M: mod_int::Mod> FPSOps<M> {
    pub fn add(&self, mut a: Vec<mod_int::ModInt<M>>, mut b: Vec<mod_int::ModInt<M>>) -> Vec<mod_int::ModInt<M>> {
        if a.len() < b.len() {
            std::mem::swap(&mut a, &mut b);
        }
        for i in 0..b.len() {
            a[i] += b[i];
        }
        a
    }
    pub fn mul(&self, a: Vec<mod_int::ModInt<M>>, b: Vec<mod_int::ModInt<M>>) -> Vec<mod_int::ModInt<M>> {
        type MInt<M> = mod_int::ModInt<M>;
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let n = a.len() - 1;
        let m = b.len() - 1;
        let mut p = 1;
        while p <= n + m { p *= 2; }
        let mut f = vec![MInt::new(0); p];
        let mut g = vec![MInt::new(0); p];
        for i in 0..n + 1 { f[i] = a[i]; }
        for i in 0..m + 1 { g[i] = b[i]; }
        let fac = MInt::new(p as i64).inv();
        let zeta = self.gen.pow((M::m() - 1) / p as i64);
        fft::fft(&mut f, zeta, 1.into());
        fft::fft(&mut g, zeta, 1.into());
        for i in 0..p { f[i] *= g[i] * fac; }
        fft::inv_fft(&mut f, zeta.inv(), 1.into());
        f.truncate(n + m + 1);
        f
    }
}
