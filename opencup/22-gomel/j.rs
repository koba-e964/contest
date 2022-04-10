use std::collections::*;
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

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

    fn mul_mod(x: i64, mut y: i64, n: i64) -> i64 {
        assert!(x >= 0);
        assert!(x < n);
        let mut sum = 0;
        let mut cur = x;
        while y > 0 {
            if (y & 1) == 1 { sum = add_mod(sum, cur, n); }
            cur = add_mod(cur, cur, n);
            y >>= 1;
        }
        sum
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

fn all_divisors(pe: &[(i64, usize)]) -> Vec<i64> {
    fn dfs(v: usize, pe: &[(i64, usize)], cur: i64, divs: &mut Vec<i64>) {
        if v >= pe.len() {
            divs.push(cur);
            return;
        }
        let (p, e) = pe[v];
        dfs(v + 1, pe, cur, divs);
        let mut c = cur * p;
        for _ in 1..e + 1 {
            dfs(v + 1, pe, c, divs);
            c = c * p;
        }
    }
    let mut divs = vec![];
    dfs(0, pe, 1, &mut divs);
    divs.sort_unstable();
    divs
}

fn nth(a: i64, n: i64) -> i64 {
    let mut pass = 0;
    let mut fail = std::cmp::min(a, 1 << ((60 + n - 1) / n)) + 1;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut tmp = 1i64;
        for _ in 0..n {
            tmp = tmp.saturating_mul(mid);
        }
        if tmp <= a {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    pass
}

fn main() {
    input! {
        t: usize,
        k: [i64; t],
    }
    for k in k {
        println!("{}", calc(k));
    }
}

fn extract(n: i64) -> Option<(i64, i64)> {
    if n <= 1 { return None; }
    for b in 1..30 {
        let x = nth(n, b);
        let mut a = 1;
        for _ in 0..b { a *= x; }
        if a == n && pollard_rho::is_prime(x) {
            return Some((x, b));
        }
    }
    None
}

fn calc(k: i64) -> usize {
    let pe = pollard_rho::factorize(k);
    let divs = all_divisors(&pe);
    // p == 2
    let mut ans = 0;
    let mut seen = HashSet::new();
    for &d in &divs {
        if d % 2 == 0 { continue; }
        for &l in &[d, d - 1] {
            if l <= 0 { continue; }
            let mut prod = l * (l + 1);
            let mut diff = 1;
            while prod % 2 == 0 {
                prod /= 2;
                diff += 1;
            }
            let mut kk = k;
            while kk % 2 == 0 {
                kk /= 2;
                diff -= 1;
            }
            if prod == kk && diff >= 0 {
                // eprintln!("l = {}, n = {}", l, 2 * l + 1);
                seen.insert(l);
            }
        }
    }
    ans += seen.len();
    // p != 2
    let mut seen = HashSet::new();
    for i in 0..divs.len() {
        let a = divs[i];
        for j in i..divs.len() {
            let ap2 = divs[j];
            for &(ap, delta) in &[(ap2 - 2, 1), (ap2 + 2, -1)] {
                if ap <= 0 || ap % a != 0 { continue; }
                let pb = ap / a;
                if let Some((p, b1)) = extract(pb) {
                    if p == 2 { continue; }
                    let b = b1 - 1;
                    let mut kp = 0;
                    let mut kk = k;
                    while kk % p == 0 {
                        kk /= p;
                        kp += 1;
                    }
                    if kk != a * ap2 { continue; }
                    let l = pb / p * a;
                    let n = l * p + delta;
                    // eprintln!("a = {}, p = {}, l = {}, n = {}, m = {}",
                    //           a, p, l, n, b - kp + 1);
                    if b >= kp {
                        seen.insert(n);
                    }
                }
            }
        }
    }
    ans += seen.len();
    ans
}
