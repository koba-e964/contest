// https://judge.yosupo.jp/submission/5155
mod pollard_rho {
    /// binary gcd
    pub fn gcd(mut x: i64, mut y: i64) -> i64 {
        if y == 0 { return x; }
        if x == 0 { return y; }
        let k = (x | y).trailing_zeros();
        y >>= k;
        x >>= x.trailing_zeros();
        while y != 0 {
            y >>= y.trailing_zeros();
            if x > y { let t = x; x = y; y = t; }
            y -= x;
        }
        x << k
    }

    fn add_mod(x: i64, y: i64, n: i64) -> i64 {
        let z = x + y;
        if z >= n { z - n } else { z }
    }

    fn mul_mod(x: i64, y: i64, n: i64) -> i64 {
        assert!(x >= 0);
        assert!(x < n);
        let prod = x as i128 * y as i128;
        (prod % n as i128) as i64
    }

    fn mod_pow(x: i64, mut e: i64, n: i64) -> i64 {
        let mut prod = if n == 1 { 0 } else { 1 };
        let mut cur = x % n;
        while e > 0 {
            if (e & 1) == 1 { prod = mul_mod(prod, cur, n); }
            e >>= 1;
            if e > 0 { cur = mul_mod(cur, cur, n); }
        }
        prod
    }

    pub fn is_prime(n: i64) -> bool {
        if n <= 1 { return false; }
        let small = [2, 3, 5, 7, 11, 13];
        if small.iter().any(|&u| u == n) { return true; }
        if small.iter().any(|&u| n % u == 0) { return false; }
        let mut d = n - 1;
        let e = d.trailing_zeros();
        d >>= e;
        // https://miller-rabin.appspot.com/
        let a = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
        a.iter().all(|&a| {
            if a % n == 0 { return true; }
            let mut x = mod_pow(a, d, n);
            if x == 1 { return true; }
            for _ in 0..e {
                if x == n - 1 {
                    return true;
                }
                x = mul_mod(x, x, n);
                if x == 1 { return false; }
            }
            x == 1
        })
    }

    fn pollard_rho(n: i64, c: &mut i64) -> i64 {
        // An improvement with Brent's cycle detection algorithm is performed.
        // https://maths-people.anu.edu.au/~brent/pub/pub051.html
        if n % 2 == 0 { return 2; }
        loop {
            let mut x: i64; // tortoise
            let mut y = 2; // hare
            let mut d = 1;
            let cc = *c;
            let f = |i| add_mod(mul_mod(i, i, n), cc, n);
            let mut r = 1;
            // We don't perform the gcd-once-in-a-while optimization
            // because the plain gcd-every-time algorithm appears to
            // outperform, at least on judge.yosupo.jp :)
            while d == 1 {
                x = y;
                for _ in 0..r {
                    y = f(y);
                    d = gcd((x - y).abs(), n);
                    if d != 1 { break; }
                }
                r *= 2;
            }
            if d == n {
                *c += 1;
                continue;
            }
            return d;
        }
    }

    /// Outputs (p, e) in p's ascending order.
    pub fn factorize(x: i64) -> Vec<(i64, usize)> {
        if x <= 1 { return vec![]; }
        let mut hm = std::collections::HashMap::new();
        let mut pool = vec![x];
        let mut c = 1;
        while let Some(u) = pool.pop() {
            if is_prime(u) {
                *hm.entry(u).or_insert(0) += 1;
                continue;
            }
            let p = pollard_rho(u, &mut c);
            pool.push(p);
            pool.push(u / p);
        }
        let mut v: Vec<_> = hm.into_iter().collect();
        v.sort();
        v
    }
} // mod pollard_rho
