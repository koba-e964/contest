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
    let s: Vec<_> = get_word().chars().collect();
    let mut freq = vec![0; 256];
    for &c in s.iter() {
        freq[(c as u8) as usize] += 1;
    }
    let mut mi = freq[b'b' as usize];
    mi = min(mi, freq[b'B' as usize]);
    mi = min(mi, freq[b'a' as usize] / 2);
    mi = min(mi, freq[b'l' as usize]);
    mi = min(mi, freq[b'u' as usize] / 2);
    mi = min(mi, freq[b'r' as usize]);
    mi = min(mi, freq[b's' as usize]);
    println!("{}", mi);
}
