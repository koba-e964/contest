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
            if res.is_err() || u8b[0] <= ' ' as u8 {
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

fn simulate() -> i32 {
    let n: i32 = get();
    let m: usize = get();
    let a: i32 = get();
    let b: i32 = get();
    let mut cur = n as i32;
    for i in 0 .. m {
        if cur <= a {
            cur += b;
        }
        let c: i32 = get();
        cur -= c;
        if cur < 0 {
            return i as i32 + 1;
        }
    }
    return -1;
}

fn main() {
    let res = simulate();
    println!("{}", if res == -1 { "complete".to_string() } else { format!("{}", res) });
}
