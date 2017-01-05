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
    let mut td: Vec<((i64, i64), usize)> = vec![((0, 0), 0); n];
    for i in 0 .. n {
        let t = get();
        (td[i].0).0 = t;
        td[i].1 = i + 1;
    }
    for i in 0 .. n {
        let d = get();
        (td[i].0).1 = d;
    }
    td.sort_by(|&((t1, d1), _), &((t2, d2), _)| (t2 * d1).cmp(&(t1 * d2)));
    for i in 0 .. n {
        print!("{}{}", td[i].1, if i == n - 1 { "\n" } else { " " });
    }
}
