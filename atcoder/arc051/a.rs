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

fn main() {
    let x1: i32 = get();
    let y1: i32 = get();
    let r: i32 = get();
    let mut x2: i32 = get();
    let mut y2: i32 = get();
    let mut x3: i32 = get();
    let mut y3: i32 = get();
    x2 -= x1;
    x3 -= x1;
    y2 -= y1;
    y3 -= y1;
    println!("{}", if y2 <= -r && r <= y3 && x2 <= -r && r <= x3 { "NO" } else { "YES" } );
    println!("{}", if max(x2.abs(), x3.abs()).pow(2) + max(y2.abs(), y3.abs()).pow(2) <= r * r { "NO" } else { "YES" });
}
