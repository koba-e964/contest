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
    let mut e = Vec::new();
    let mut link = vec![Vec::new(); n];
    for _ in 0 .. m {
        let a: usize = get();
        let b: usize = get();
        link[a].push(e.len());
        link[b].push(e.len());
        e.push(Some((a, b)));
    }
    let mut ans = vec![-1; n];
    for i in (0 .. n).rev() {
        let mut usei = false;
        for &ei in link[i].iter() {
            if let Some((x, _)) = e[ei] {
                if x == i {
                    usei = true;
                }
            }
        }
        if ! usei {
            ans[i] = 0;
            continue;
        }
        ans[i] = 1;
        // eliminate all edges connected to i
        for &ei in link[i].iter() {
            e[ei] = None;
        }
    }
    let mut ma = 0;
    for i in 0 .. n {
        if ans[i] == 1 {
            ma = i;
        }
    }
    for i in (0 .. ma + 1).rev() {
        print!("{}", ans[i]);
    }
    println!("");
}
