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
    let n: i64 = get();
    let m: i64 = get();
    let nn = n as usize;
    let mm = m as usize;
    let k: i64 = get();
    let x: usize = get::<usize>() - 1;
    let y: usize = get::<usize>() - 1;
    let mut tbl = vec![vec![0; mm]; nn];
    let div = if n == 1 { m } else { 2 * (n - 1) * m };
    let ascension = k / div;
    let rem = k % div;
    for i in 0 .. nn {
        for j in 0 .. mm {
            tbl[i][j] = ascension * if i == 0 || i == nn - 1 { 1 } else { 2 };
        }
    }
    for i in 0 .. (rem as usize) {
        let p = i / mm;
        let q = i % mm;
        tbl[if p >= nn { 2 * nn - 2 - p } else { p }][q] += 1;
    }
    let mut ma = 0;
    let mut mi = tbl[0][0];
    for i in 0 .. nn {
        ma = max(ma, *tbl[i].iter().max().unwrap());
        mi = min(mi, *tbl[i].iter().min().unwrap());
    }
    println!("{} {} {}", ma, mi, tbl[x][y]);
}
