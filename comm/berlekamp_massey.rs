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

fn polymul(a: &[MInt], b: &[MInt], mo: &[MInt]) -> Vec<MInt> {
    let n = a.len();
    debug_assert_eq!(b.len(), n);
    debug_assert_eq!(mo.len(), n + 1);
    debug_assert_eq!(mo[n], 1.into());
    let mut ret = vec![MInt::new(0); 2 * n - 1];
    for i in 0..n {
        for j in 0..n {
            ret[i + j] += a[i] * b[j];
        }
    }
    for i in (n..2 * n - 1).rev() {
        let val = ret[i];
        for j in 0..n {
            ret[i - n + j] -= val * mo[j];
        }
    }
    ret[..n].to_vec()
}
    

fn polypow(a: &[MInt], mut e: i64, mo: &[MInt]) -> Vec<MInt> {
    let n = a.len();
    debug_assert_eq!(mo.len(), n + 1);
    let mut prod = vec![MInt::new(0); n];
    prod[0] += 1;
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            prod = polymul(&prod, &cur, mo);
        }
        cur = polymul(&cur, &cur, mo);
        e /= 2;
    }
    prod
}

// Finds u a^e v^T by using Berlekamp-massey algorithm.
fn eval_matpow(a: &[Vec<MInt>], e: i64, u: &[MInt], v: &[MInt]) -> MInt {
    let k = a.len();
    // Find first 2k terms
    let mut terms = vec![MInt::new(0); 2 * k];
    let mut cur = u.to_vec();
    for pos in 0..2 * k {
        for i in 0..k {
            terms[pos] += cur[i] * v[i];
        }
        let mut nxt = vec![MInt::new(0); k];
        for i in 0..k {
            for j in 0..k {
                nxt[j] += cur[i] * a[i][j];
            }
        }
        cur = nxt;
    }
    let mut poly = berlekamp_massey(k, &terms);
    poly.reverse();
    // If terms' minimal polynomial is 1, then terms == 0.
    if poly.len() == 1 {
        return MInt::new(0);
    }
    // If terms' minimal polynomial is x + r,
    // then terms is a gemetric progression with common ratio -r.
    // terms[i] = a * (-r)^i.
    if poly.len() == 2 {
        let r = -poly[0];
        return terms[0] * r.pow(e);
    }
    let mut base = vec![MInt::new(0); poly.len() - 1];
    base[1] += 1;
    let powpoly = polypow(&base, e, &poly);
    let mut ans = MInt::new(0);
    for i in 0..poly.len() - 1 {
        ans += powpoly[i] * terms[i];
    }
    ans
}
