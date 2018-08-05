#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read};
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

mod pollard_rho {
    use std::collections::HashMap;
    /// binary gcd
    pub fn gcd(mut x: i64, mut y: i64) -> i64 {
        if y == 0 { return x; }
        if x == 0 { return y; }
        let mut sh = 0;
        while ((x | y) & 1) == 0 {
            x >>= 1; y >>= 1; sh += 1;
        }
        while (x & 1) == 0 { x >>= 1; }
        while y != 0 {
            while (y & 1) == 0 { y >>= 1; }
            if x > y { let t = x; x = y; y = t; }
            y -= x;
        }
        x << sh
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
            if (y & 1) == 1 {
                sum = add_mod(sum, cur, n);
            }
            cur = add_mod(cur, cur, n);
            y >>= 1;
        }
        sum
    }

    fn mod_pow(x: i64, mut e: i64, n: i64) -> i64 {
        let mut prod = if n == 1 { 0 } else { 1 };
        let mut cur = x % n;
        while e > 0 {
            if (e & 1) == 1 {
                prod = mul_mod(prod, cur, n);
            }
            cur = mul_mod(cur, cur, n);
            e >>= 1;
        }
        prod
    }

    pub fn is_prime(n: i64) -> bool {
        if n <= 1 { return false; }
        let small = [2, 3, 5, 7, 11, 13];
        if small.iter().any(|&u| u == n) { return true; }
        if small.iter().any(|&u| n % u == 0) { return false; }
        let mut d = n - 1;
        let mut e = 0;
        while (d & 1) == 0 {
            d >>= 1;
            e += 1;
        }
        let a = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
        a.iter().all(|&a| {
            if a >= n { return true; }
            let mut x = mod_pow(a, d, n);
            if x == 1 { return true; }
            for _ in 0 .. e {
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
        if n % 2 == 0 { return 2; }
        loop {
            let mut x: i64 = 2;
            let mut y = 2;
            let mut d = 1;
            let cc = *c;
            let f = |i| add_mod(mul_mod(i, i, n), cc, n);
            while d == 1 {
                x = f(x);
                y = f(f(y));
                d = gcd((x - y).abs(), n);
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
        if x <= 1 {
            return Vec::new();
        }
        let mut hm = HashMap::new();
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


use pollard_rho::*;

fn solve() {
    let n = get();
    let x: i64 = get();
    let y: i64 = get();
    if y % x != 0 {
        println!("0");
        return;
    }
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let b: Vec<i64> = a.iter().filter_map(|&a| if a % x == 0 { Some(a / x) } else { None }).collect();
    let c: Vec<i64> = a.iter().filter_map(|&a| if y % a == 0 { Some(y / a) } else { None }).collect();
    let y = y / x;
    let fac = factorize(y);
    let m = fac.len();
    let mut pat1 = vec![0i64; 1 << m];
    let mut pat2 = vec![0i64; 1 << m];
    for &b in &b {
        let mut hot = 0;
        for i in 0 .. m {
            let p = fac[i].0;
            if b % p == 0 {
                hot |= 1 << i;
            }
        }
        pat1[hot] += 1;
    }
    for &c in &c {
        let mut hot = 0;
        for i in 0 .. m {
            let p = fac[i].0;
            if c % p == 0 {
                hot |= 1 << i;
            }
        }
        pat2[hot] += 1;
    }
    // zeta
    for i in 0 .. m {
        for bits in 0 .. 1 << m {
            if (bits & 1 << i) != 0 {
                pat2[bits] += pat2[bits ^ 1 << i];
            }
        }
    }
    let mut ans = 0;
    for bits in 0 .. 1 << m {
        ans += pat1[bits] * pat2[(1 << m) - 1 - bits];
    }
    println!("{}", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
