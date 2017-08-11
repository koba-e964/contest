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
    let n: usize = get();
    let m: usize = get();
    let k: usize = get();
    let mut edges = vec![0u32; n];
    for _ in 0 .. m {
        let a: usize = get();
        let b: usize = get();
        edges[a] |= 1 << b;
        edges[b] |= 1 << a;
    }
    // O(2^n * n)
    let mut sol = 1 << n;
    for bits in 0 .. 1u32 << n {
        if bits.count_ones() != k as u32 {
            continue;
        }
        let mut ok = true;
        for i in 0 .. n {
            if (bits & 1 << i) == 0 { continue; }
            if (bits & edges[i]) != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            sol = bits;
            break;
        }
    }
    for i in 0 .. n {
        if (sol & 1 << i) != 0 {
            println!("{}", i);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
