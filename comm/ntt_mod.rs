// Dependencies: ModInt.rs
mod ntt_mod {
    use ::mod_int::*;
    fn ntt_internal<M: Mod>(a: &mut [ModInt<M>], g: ModInt<M>) {
        let n = a.len();
        assert_eq!(n & n.wrapping_neg(), n);
        let h = g.pow((M::m() - 1) / n as i64);
        // bit reverse
        {
            let mut rev = 0;
            for j in 0 .. n {
                if j < rev { a.swap(j, rev); }
                let mut k = n >> 1;
                loop {
                    rev ^= k;
                    if k <= rev { break; }
                    k >>= 1;
                }
            }
        }
        let mut m = 1;
        while m < n {
            let m2 = 2 * m;
            let base = h.pow(n as i64 / m2 as i64);
            let mut r = 0;
            while r < n {
                let mut w = ModInt::new(1);
                for s in r .. r + m {
                    let u = a[s];
                    let d = a[s + m] * w;
                    a[s] = u + d;
                    a[s + m] = u - d;
                    w = w * base;
                }
                r += m2;
            }
            m *= 2;
        }
    }

    pub fn ntt<M: Mod>(a: &mut [ModInt<M>], g: ModInt<M>) {
        ntt_internal(a, g);
    }

    pub fn intt<M: Mod>(a: &mut [ModInt<M>], g: ModInt<M>) {
        ntt_internal(a, g.inv());
        let n = a.len() as i64;
        let factor = ModInt::new(n).inv();
        for val in a.iter_mut() { *val = *val * factor; }
    }
} // mod ntt_mod
