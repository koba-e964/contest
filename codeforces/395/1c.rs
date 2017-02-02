#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

/**
 * p is prime
 */
fn modsqrt(mut a: i64, p: i64) -> Option<i64> {
    a %= p;
    if a == 0 {
        return Some(0);
    }
    if p == 2 {
        return Some(a);
    }
    if powmod(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    let mut b = 1;
    while powmod(b, (p - 1) / 2, p) == 1 {
        b += 1;
    }
    let mut e = 0;
    let mut m = p - 1;
    while m % 2 == 0 {
        m /= 2;
        e += 1;
    }
    let mut x = powmod(a, (m - 1) / 2, p);
    let mut y = a * (x * x % p) % p;
    x = x * a % p;
    let mut z = powmod(b, m, p);
    while y != 1 {
        let mut j = 0;
        let mut t = y;
        while t != 1 {
            j += 1;
            t = t * t % p;
        }
        assert!(j < e);
        z = powmod(z, 1 << (e - j - 1), p);
        x = x * z % p;
        z = z * z % p;
        y = y * z % p;
        e = j;
    }
    Some(x)
}

fn main() {
    let m: i64 = get();
    let n = get();
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    if m == 2 {
        // count 0 and 1
        let mut zero: i64 = 0;
        let mut one: i64 = 0;
        for i in 0 .. n {
            if a[i] == 0 {
                zero += 1;
            } else {
                one += 1;
            }
        }
        if zero == 0 {
            println!("1 0");
            return;
        }
        if one == 0 {
            println!("0 0");
            return;
        }
        if zero - one >= 0 && zero - one <= 1 {
            println!("0 1");
            return;
        }
        if zero - one == -1 {
            println!("1 1");
            return;
        }
        println!("-1");
        return;
    }
    if m == 3 {
        if n == 1 {
            println!("{} 0", a[0]);
            return;
        }
        if n == 2 {
            println!("{} {}", a[0], (a[0] - a[1] + 3) % 3);
            return;
        }
        println!("0 1");
        return;
    }
    if n as i64 == m {
        println!("0 1");
        return;
    }
    if n as i64 + 1 == m {
        let mut k = -1; // k is missing
        let sa: HashSet<_> = a.iter().collect();
        for i in 0 .. m {
            if !sa.contains(&i) {
                k = i;
            }
        }
        println!("{} 1", (k + 1) % m);
        return;
    }
    if n == 1 {
        println!("{} 0", a[0]);
        return;
    }
    let mut avrg: i64 = 0;
    let mut sqsum = 0;
    for &v in a.iter() {
        avrg += v;
        avrg %= m;
        sqsum += v * v;
        sqsum %= m;
    }
    // m is prime
    avrg = avrg * powmod(n as i64, m - 2, m) % m;
    //println!("avrg = {}", avrg);
    sqsum += m - (n as i64 * avrg % m) * avrg % m;
    sqsum %= m;
    // m >= 5, n / 12 * (n^2 - 1) is coprime with m
    sqsum = sqsum * 12 % m;
    sqsum = sqsum * powmod(n as i64, m - 2, m) % m;
    let dsq = sqsum * powmod(n as i64 * n as i64 - 1, m - 2, m) % m;
    //println!("d^2 = {} mod {}", dsq, m);
    let d = modsqrt(dsq, m);
    if d == None {
        println!("-1");
        return;
    }
    let d = d.unwrap();
    let x = avrg + ((n as i64 - 1) * (m + 1) / 2) % m * (m - d) % m;
    let x = x % m;

    // check if it is real
    let sa: HashSet<_> = a.iter().cloned().collect();
    for i in 0 .. n {
        let virt = (x + (i as i64) * d) % m;
        if !sa.contains(&virt) {
            println!("-1");
            return;
        }
    }
    println!("{} {}", x, d);
}
