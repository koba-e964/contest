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
    let k: usize = get();
    let mut ary: Vec<usize> = vec![0; n];
    for i in 0 .. n {
        ary[i] = i + 1;
    }
    for _ in 0 .. k {
        let x = get::<usize>() - 1;
        let y = get::<usize>() - 1;
        ary.swap(x, y);
    }
    let ans: Vec<usize> = (0 .. n).map(|_| get()).collect();
    let mut ans_inv: Vec<usize> = vec![0; n];
    for i in 0 .. n {
        ans_inv[ans[i] - 1] = i + 1;
    }
    let mut ops: Vec<usize> = Vec::new();
    for i in 0 .. n {
        // find ans[i] in ary and move it to the left
        let mut pos = i;
        loop {
            assert!(pos < n);
            if ary[pos] == ans_inv[i] {
                break;
            }
            pos += 1;
        }
        // pos -> i
        for j in (i .. pos).rev() {
            ops.push(j + 1);
            ary.swap(j, j + 1);
        }
    }
    println!("{}", ops.len());
    for v in ops {
        println!("{} {}", v, v + 1);
    }
}
