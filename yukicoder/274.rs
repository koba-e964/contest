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

fn main() {
    let n: usize = get();
    let m: i32 = get();
    let mut lr: Vec<(i32, i32)> = vec![(0, 0); 2 * n];
    let mut pool: Vec<(i32, i32, usize)> = Vec::new();
    for i in 0 .. n {
        lr[i].0 = get();
        lr[i].1 = get();
        pool.push((lr[i].0, lr[i].1, i));
        pool.push((m - 1 - lr[i].1, m - 1 - lr[i].0, i + n));
    }
    pool.sort();
    let mut occ: Vec<bool> = vec![false; m as usize];
    let mut used: Vec<bool> = vec![false; n];
    for (u, v, idx) in pool {
        if used[idx % n] {
            continue;
        }
        let mut can_fill: bool = true;
        for i in u .. (v + 1) {
            if occ[i as usize] {
                can_fill = false;
            }
        }
        if can_fill {
            used[idx % n] = true;
            for i in u .. (v + 1) {
                occ[i as usize] = true;
            }
        }
    }
    println!("{}", if used.iter().all(|&x| x) { "YES" } else { "NO" });
}
