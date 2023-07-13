// Depends on MInt.rs
// Verified by: https://yukicoder.me/submissions/890380
// C(a, b)
fn comb(a: i64, b: i64) -> MInt {
    if a < 0 {
        return MInt::new(0);
    }
    let b = std::cmp::min(b, a - b);
    if b < 0 {
        return MInt::new(0);
    }
    let mut num = MInt::new(1);
    let mut den = MInt::new(1);
    for i in 1..b + 1 {
        num *= a + 1 - i;
        den *= i;
    }
    num * den.inv()
}

// \sum_{l <= i <= r} coef[j] i^j C(i, a)
fn comb_poly_sum(rng: std::ops::RangeInclusive<i64>, a: i64, coef: &[MInt]) -> MInt {
    // Stirling numbers of second kind, with S2(n, k) negative for (n + k) % 2 != 0
    // Complexity: O(n^2)
    // First terms are:
    // 1
    // -1 1
    // 1 -3 1
    // -1 7 -6 1
    let n = coef.len();
    let mut stir = vec![vec![MInt::new(0); n + 1]; n + 1];
    stir[0][0] += 1;
    for i in 1..n + 1 {
        for j in 1..i + 1 {
            let mut me = stir[i - 1][j - 1];
            me -= stir[i - 1][j] * j as i64;
            stir[i][j] = me;
        }
    }
    let mut trans = vec![MInt::new(0); n];
    for i in 0..n {
        for j in 0..i + 1 {
            trans[j] += stir[i + 1][j + 1] * coef[i];
        }
    }
    let mut tot = MInt::new(0);
    let (l, r) = rng.into_inner();
    let mut cur = MInt::new(1); // (a+1)...(a+i)
    for i in 0..n {
        tot += trans[i] * (comb(r + 1 + i as i64, a + i as i64 + 1) - comb(l + i as i64, a + i as i64 + 1)) * cur;
        cur *= a + i as i64 + 1;
    }
    tot
}
