#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
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

fn ith(i: usize) -> i64 {
    println!("? {}", i + 1);
    let r: i64 = get();
    if r == -1 {
        std::process::exit(0);
    }
    r
}

fn gt(x: i64) -> bool {
    if x > 1_000_000_000 {
        return false;
    }
    if x < 0 {
        return true;
    }
    println!("> {}", x);
    let r: i32 = get();
    if r == -1 {
        std::process::exit(0);
    }
    r == 1
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y > 0 {
        let r = x % y;
        x = y; y = r;
    }
    x
}

fn solve() {
    let n: usize = get();
    // Find max(a)
    let ma;
    {
        let mut pass: i64 = 1 << 30;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if gt(mid) {
                fail = mid;
            } else {
                pass = mid;
            }
        }
        ma = pass;
    }
    // randomly pick 30 elements
    const MOD: u64 = 1_000_000_007;
    let hm: HashMap<i32, i32> = HashMap::new();
    use std::hash::{Hasher, BuildHasher};
    let mut hash = hm.hasher().build_hasher();
    hash.write_u32(8128);
    let mut x: u64 = hash.finish() % MOD;
    let a = 13333;
    let b = 4247;
    let mut g = 0;
    for _ in 0..30 {
        x = (a * x + b) % MOD;
        let idx = (x % n as u64) as usize;
        let val = ith(idx);
        g = gcd(g, ma - val);
    }
    let d = g;
    let x = ma - (n as i64 - 1) * d;
    println!("! {} {}", x, d);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
