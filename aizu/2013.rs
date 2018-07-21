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

fn to_time(v: Vec<u8>) -> usize {
    let h: usize = (10 * (v[0] - 48) + v[1] - 48).into();
    let m: usize = (10 * (v[3] - 48) + v[4] - 48).into();
    let s: usize = (10 * (v[6] - 48) + v[7] - 48).into();
    3600 * h + 60 * m + s
}

fn solve() {
    loop {
        let n = get();
        if n == 0 { break; }
        const T: usize = 86400;
        let mut imos = [0; T];
        for _ in 0 .. n {
            let a = to_time(get_word().bytes().collect::<Vec<_>>());
            let b = to_time(get_word().bytes().collect::<Vec<_>>());
            imos[a] += 1;
            imos[b] -= 1;
        }
        for i in 0 .. T - 1 {
            imos[i + 1] += imos[i];
        }
        let ma = imos.iter().max().unwrap();
        println!("{}", ma);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
