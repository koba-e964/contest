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
    let _n: usize = get();
    let mut sher: Vec<char> = get_word().chars().collect();
    let mut mori: Vec<char> = get_word().chars().collect();
    sher.sort();
    mori.sort();
    let mut mi_flick = 0;
    let mut mori2 = mori.clone();
    for &v in sher.iter() {
        // get the minimum mori[?] >=v
        let mut found = false;
        for i in 0 .. mori.len() {
            if mori[i] >= v {
                found = true;
                mori.remove(i);
                break;
            }
        }
        if !found { mi_flick += 1; }
    }
    let mut ma_flick = 0;
    for &v in sher.iter() {
        // get the minimum mori[?] >v
        for i in 0 .. mori2.len() {
            if mori2[i] > v {
                ma_flick += 1;
                mori2.remove(i);
                break;
            }
        }
    }
    println!("{}\n{}", mi_flick, ma_flick);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
