let count_matching_with_size = |a: usize, b: usize, k: usize| {
    if a < k || b < k {
        MInt::new(0)
    } else {
        fac[a] * invfac[k] * invfac[a - k] * fac[b] * invfac[b - k]
    }
};
