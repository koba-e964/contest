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
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
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
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut ans: Vec<i64> = vec![0; n];
    let mut delta: Vec<i64> = vec![0; n];
    {
        let mut cur = 0;
        for i in 0 .. n {
            if cur < a[i] {
                delta[i] = a[i] - cur;
                cur = a[i];
            }
        }
    }
    let mut srt = Vec::new();
    for i in 0 .. n {
        srt.push((a[i], i));
    }
    srt.push((0, 114514));
    srt.sort();
    srt.reverse();
    let mut mi = n;
    for i in 0 .. srt.len() - 1 {
        let diff = srt[i].0 - srt[i + 1].0;
        mi = min(mi, srt[i].1);
        ans[mi] += diff * (i + 1) as i64;
    }
    for i in 0 .. n {
        println!("{}", ans[i]);
    }
}
