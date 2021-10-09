// Verified by: https://yukicoder.me/submissions/706484
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let r = a % b;
    let q = a / b;
    let (g, x, y) = ext_gcd(b, r);
    (g, y, x - q * y)
}

fn inv_mod(a: i64, b: i64) -> i64 {
    let (_, mut x, _) = ext_gcd(a, b);
    x %= b;
    if x < 0 {
        x += b;
    }
    x
}

// gcd(rm[i].1, rm[j].1) == 1 for i != j
// Ref: https://www.creativ.xyz/ect-gcd-crt-garner-927/
// O(n^2)
fn garner(rm: &[(i64, i64)], mo: i64) -> i64 {
    let n = rm.len();
    let mut x_mo = (rm[0].0 % rm[0].1) % mo;
    let mut mp_mo = 1;
    let mut coef = Vec::with_capacity(n);
    coef.push(rm[0].0 % rm[0].1);
    for i in 1..n {
        let (r, m) = rm[i];
        let r = r % m;
        let mut mp_mi = 1;
        let mut x_mi = 0;
        mp_mo = mp_mo * (rm[i - 1].1 % mo) % mo;
        for j in 0..i {
            x_mi = (x_mi + mp_mi * (coef[j] % m)) % m;
            mp_mi = mp_mi * (rm[j].1 % m) % m;
        }
        let t = (r - x_mi + m) % m * inv_mod(mp_mi, m) % m;
        x_mo = (x_mo + t % mo * mp_mo) % mo;
        coef.push(t);
    }
    x_mo
}
