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
    let mut col: [u8; 4] = [0, 0, 0, 0];
    let mut col_no = HashMap::<u8, usize>::new();
    col_no.insert(b'R', 0);
    col_no.insert(b'B', 1);
    col_no.insert(b'Y', 2);
    col_no.insert(b'G', 3);
    let s: Vec<u8> = get_word().bytes().collect();
    for i in 0 .. s.len() {
        if s[i] != b'!' {
            col[i % 4] = s[i];
        }
    }
    let mut nums = [0usize; 4];
    for i in 0 .. s.len() {
        if s[i] == b'!' {
            nums[*col_no.get(&col[i % 4]).unwrap()] += 1;
        }
    }
    for i in 0 .. 4 {
        print!("{}{}", nums[i], if i == 3 { "\n" } else { " " });
    }
}
