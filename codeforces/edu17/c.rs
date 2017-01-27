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
    let a: Vec<_> = get_word().bytes().collect();
    let b: Vec<_> = get_word().bytes().collect();
    let n = a.len();
    let m = b.len();
    let mut dpl = vec![0; n + 1];
    let mut dpr = vec![m; n + 1];
    for i in 0 .. n {
        let tmp = dpl[i];
        if tmp < m && b[tmp] == a[i] {
            dpl[i + 1] = tmp + 1;
        } else {
            dpl[i + 1] = tmp;
        }
    }
    for i in (0 .. n).rev() {
        let tmp = dpr[i + 1];
        if tmp > 0 && b[tmp - 1] == a[i] {
            dpr[i] = tmp - 1;
        } else {
            dpr[i] = tmp;
        }
    }
    let mut mi = m as i32 + 1;
    let mut mini = 0;
    for i in 0 .. n + 1 {
        if mi > dpr[i] as i32 - dpl[i] as i32 {
            mi = dpr[i] as i32 - dpl[i] as i32;
            mini = i;
        }
    }
    if mi == m as i32 {
        println!("-");
    } else {
        println!("{}{}", String::from_utf8(b[0 .. dpl[mini]].to_vec()).unwrap(),
                 String::from_utf8(b[max(dpr[mini], dpl[mini]) .. m].to_vec()).unwrap());
    }
}
