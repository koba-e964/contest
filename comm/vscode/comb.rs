let comb = |x: usize, y: usize| {
    if x >= y {
        fac[x] * invfac[y] * invfac[x - y]
    } else {
        0.into()
    }
};
