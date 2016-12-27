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

fn ord(c: char) -> usize {
    (c as usize) - 97
}

fn chr(n: usize) -> char {
    (n + 97) as u8 as char
}

fn main() {
    let s: Vec<char> = get_word().chars().collect();
    let t: Vec<char> = get_word().chars().collect();
    let n = s.len();
    let mut tbl: Vec<HashSet<usize>> = vec![HashSet::new(); 26];
    for i in 0 .. n {
        let u = ord(s[i]);
        let v = ord(t[i]);
        tbl[u].insert(v);
        tbl[v].insert(u);
    }
    for i in 0 .. 26 {
        if tbl[i].len() >= 2 {
            println!("-1");
            return;
        }
    }
    let mut ans: Vec<(usize, usize)> = Vec::new();
    for i in 0 .. 26 {
        if let Some(&w) = tbl[i].iter().next() {
            if i < w {
                ans.push((i, w));
            }
        }
    }
    println!("{}", ans.len());
    for (u, v) in ans {
        println!("{} {}", chr(u), chr(v));
    }
}
