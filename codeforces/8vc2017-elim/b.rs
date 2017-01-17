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
    let n = get();
    let m = get();
    let mut sa: HashSet<_> = (0 .. n).map(|_| get_word()).collect();
    let mut sb: HashSet<_> = (0 .. m).map(|_| get_word()).collect();
    let mut inter: HashSet<String> = sa.intersection(&sb).cloned().collect();
    loop {
        // A
        if !inter.is_empty() {
            let v: String = inter.iter().cloned().next().unwrap();
            inter.remove(&v);
            sa.remove(&v);
            sb.remove(&v);
        } else if !sa.is_empty() {
            let v = sa.iter().cloned().next().unwrap();
            sa.remove(&v);
        } else {
            println!("NO");
            return;
        }
        // B
        if !inter.is_empty() {
            let v: String = inter.iter().cloned().next().unwrap();
            inter.remove(&v);
            sa.remove(&v);
            sb.remove(&v);
        } else if !sb.is_empty() {
            let v = sb.iter().cloned().next().unwrap();
            sb.remove(&v);
        } else {
            println!("YES");
            return;
        }
    }
}
