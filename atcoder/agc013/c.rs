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

fn solve() {
    let n = get();
    let l: i64 = get();
    let t: i64 = get();
    let mut x = vec![0i64; n];
    let mut w = vec![0i64; n];
    for i in 0 .. n {
        x[i] = get();
        w[i] = if get::<usize>() == 2 { -1 } else { 1 };
    }
    let mut flipped = false;
    if w[0] == -1 {
        for i in 0 .. n {
            w[i] = -w[i];
            x[i] = (l - x[i]) % l;
        }
        flipped = true;
    }
    assert_eq!(w[0], 1);
    let mut enc = 0; // #encounters
    for i in 1 .. n {
        if w[i] == -1 {
            enc += 2 * t / l;
            if (2 * t) % l >= (x[i] - x[0] + l) % l {
                enc += 1;
            }
        }
    }
    let mut pool = Vec::new();
    for i in 0 .. n {
        let nova = (x[i] + w[i] * (t % l) + l) % l;
        pool.push((nova, w[i])); // -1 comes first
    }
    pool.sort();
    // Where did Ant enc fall?
    enc %= n as i64;
    let t = ((x[0] + w[0] * (t % l) + l) % l, w[0]);
    let mut idx = 0;
    while idx < n {
        if pool[idx] == t {
            break;
        }
        idx += 1;
    }
    assert!(idx < n);
    // enc -> idx
    for i in 0 .. n {
        let target = if flipped {
            (n - i + idx + n - enc as usize) % n
        } else {
            (i + idx + n - enc as usize) % n
        };
        let mut x = pool[target].0;
        if flipped {
            x = (l - x) % l;
        }
        println!("{}", x);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
