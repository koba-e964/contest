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

fn solve() {
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let q: usize = get();
        let c: Vec<u8> = get_word().bytes().collect();
        let mut acc = vec![0; n + 1];
        for i in 0..n {
            acc[i + 1] = acc[i] + (c[i] - b'0') as usize;
        }
        for _ in 0..q {
            let l = get::<usize>() - 1;
            let r: usize = get();
            let ok = acc[l] != l * (1 - (acc[l + 1] - acc[l]));
            let ok = ok || acc[n] - acc[r]
                != (n - r) * (1 - (acc[r] - acc[r - 1]));
            println!("{}", if ok { "YES" } else { "NO" });
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
