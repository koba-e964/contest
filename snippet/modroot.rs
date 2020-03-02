// https://yukicoder.me/submissions/437269
// Baby-step giant-step. https://codeforces.com/contest/1106/submission/49502575
// Find x s.t. x^exp = m
// Depends on: mod_arith.rs
fn modroot(m: i64, exp: i64, p: i64) -> Option<i64> {
    // prime factors of p - 1
    let mut rel = vec![];
    {
        let mut v = p - 1;
        let mut q = 2;
        while v > 1 && v >= q * q {
            if v % q == 0 {
                rel.push(q);
                while v % q == 0 {
                    v /= q;
                }
            }
            q += 1;
        }
        if v > 1 {
            rel.push(v);
        }
    }
    let mut gen = 2;
    loop {
        let ok = rel.iter().all(|q| powmod(gen, (p - 1) / q, p) != 1);
        if ok {
            break;
        }
        gen += 1;
    }
    const SQRT: i64 = 40000;
    let mut cur = 1;
    let prog = powmod(gen, SQRT, p);
    let mut tbl = HashMap::new();
    for i in 0..SQRT {
        tbl.insert(cur, SQRT * i);
        cur = cur * prog % p;
    }
    let discrete_log = |x: i64| {
        let mut cur = x;
        for i in 0..SQRT {
            let key = cur;
            if let Some(&y) = tbl.get(&key) {
                return (y - i + p - 1) % (p - 1);
            }
            cur = cur * gen % p;
        }
        unreachable!();
    };
    let logm = discrete_log(m);
    let (g, inv, _) = extgcd(exp, p - 1);
    if logm % g != 0 {
        return None;
    }
    let inv = (inv + p - 1) % (p - 1);
    let ans = (logm / g * inv) % (p - 1);
    Some(powmod(gen, ans, p))
}
