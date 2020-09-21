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

fn gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let q = a / b;
    let (g, x, y) = gcd(b, a - q * b);
    (g, y, -q * y + x)
}

fn crt(x0: i64, m0: i64, x1: i64, m1: i64) -> i64 {
    let (_, a, b) = gcd(m0, m1);
    let mut r = (x0 * b * m1 + x1 * a * m0) % (m0 * m1);
    if r < 0 {
        r += m0 * m1;
    }
    r
}

fn ask(x: usize, y: usize) -> usize {
    println!("? {} {}", x + 1, y + 1);
    let a: usize = get();
    a
}

// This solution is not correct in that for e.g. n = 3 it fails with probability 1/3. 
fn solve() {
    let n: usize = get();
    let mut p = vec![0; n];
    for i in 0..n {
        p[i] = i;
    }
    {
        use std::hash::{Hasher, BuildHasher};
        let a = 0xdead_c0de_0013_3331;
        let b = 2457;
        let hm: HashMap<i32, i32> = HashMap::new();
        let mut hash = hm.hasher().build_hasher();
        hash.write_u32(8128);
        let mut x: u64 = hash.finish();
        for i in 0..n {
            x = x.wrapping_mul(a).wrapping_add(b);
            let val = x ^ x << 10;
            let val = (val % ((i + 1) as u64)) as usize;
            p.swap(i, val);
        }
    }
    let mut fst = vec![0; n];
    let mut fstma = 0;
    for i in 1..n {
        fst[i] = ask(p[i], p[0]);
        fstma = max(fstma, fst[i]);
    }
    let v0 = fstma + 1;
    let mut x = 0;
    for i in 1..n {
        if fstma == fst[i] {
            x = i;
            break;
        }
    }
    let mut snd = vec![0; n];
    let mut sndma = 0;
    for i in 0..n {
        if x != i {
            snd[i] = ask(p[i], p[x]);
            sndma = max(sndma, snd[i]);
        }
    }
    let v1 = sndma + 1;
    assert!(v0 as i64 * v1 as i64 >= n as i64);
    let mut ans = vec![0; n];
    ans[p[0]] = v0;
    ans[p[x]] = v1;
    print!("!");
    for i in 1..n {
        if i == x { continue; }
        let mut val = crt(fst[i] as i64, v0 as i64, snd[i] as i64, v1 as i64);
        if val == 0 {
            val += v0 as i64 * v1 as i64;
        }
        ans[p[i]] = val as usize;
    }
    for i in 0..n {
        print!(" {}", ans[i]);
    }
    println!();
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
