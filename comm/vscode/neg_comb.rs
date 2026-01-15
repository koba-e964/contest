// [x^n] 1/(1-x)^deg
let neg_comb = |deg: usize, n: usize| {
    if deg == 0 {
        return if n == 0 {
            MInt::new(1)
        } else {
            MInt::new(0)
        };
    }
    fac[n + deg - 1] * invfac[n] * invfac[deg - 1]
};
