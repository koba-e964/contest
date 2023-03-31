// Verified by: yukicoder No.1112
// https://yukicoder.me/submissions/510746
// https://en.wikipedia.org/wiki/Berlekamp%E2%80%93Massey_algorithm
// Complexity: O(n^2)
// Depends on MInt.rs
fn berlekamp_massey<P: mod_int::Mod + PartialEq>(
    n: usize,
    s: &[mod_int::ModInt<P>],
) -> Vec<mod_int::ModInt<P>>{
    type ModInt<P> = mod_int::ModInt<P>;
    let mut b = ModInt::new(1);
    let mut cp = vec![ModInt::new(0); n + 1];
    let mut bp = vec![mod_int::ModInt::new(0); n];
    cp[0] = mod_int::ModInt::new(1);
    bp[0] = mod_int::ModInt::new(1);
    let mut m = 1;
    let mut l = 0;
    for i in 0..2 * n + 1 {
        assert!(i >= l);
        assert!(l <= n);
        if i == 2 * n { break; }
        let mut d = s[i];
        for j in 1..l + 1 {
            d += cp[j] * s[i - j];
        }
        if d == ModInt::new(0) {
            m += 1;
            continue;
        }
        if 2 * l > i {
            // cp -= d/b * x^m * bp
            let factor = d * b.inv();
            for j in 0..n + 1 - m {
                cp[m + j] -= factor * bp[j];
            }
            m += 1;
            continue;
        }
        let factor = d * b.inv();
        let tp = cp.clone();
        for j in 0..n + 1 - m {
            cp[m + j] -= factor * bp[j];
        }
        bp = tp;
        b = d;
        l = i + 1 - l;
        m = 1;
    }
    cp[0..l + 1].to_vec()
}

// Finds u a^e v^T by using Berlekamp-massey algorithm.
// The linear map a is given as a closure.
// Complexity: O(n^2 log e + nT(n)) where n = |u| and T(n) = complexity of a.
// Ref: https://yukicoder.me/wiki/black_box_linear_algebra
// Verified by: yukicoder No. 215 (https://yukicoder.me/submissions/854179)
// Depends on: fps/bostan_mori.rs
fn eval_matpow<F: FnMut(&[MInt]) -> Vec<MInt>>(mut a: F, e: i64, u: &[MInt], v: &[MInt]) -> MInt {
    let k = u.len();
    // Find first 2k terms
    let mut terms = vec![MInt::new(0); 2 * k];
    let mut cur = u.to_vec();
    for pos in 0..2 * k {
        for i in 0..k {
            terms[pos] += cur[i] * v[i];
        }
        cur = a(&cur);
    }
    let poly = berlekamp_massey(k, &terms);
    let mut nom = convolution(&terms[..poly.len() - 1], &poly);
    nom.truncate(poly.len() - 1);
    bostan_mori(&nom, &poly, e)
}
