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
    let k: u64 = get();
    let a: Vec<u32> = (0 .. n).map(|_| get()).collect();
    let a_max = a.iter().fold(0, |x, y| max(x, *y));
    let mut lo = 0;
    let mut hi = a_max + 1;
    let mut memo = vec![0; a_max as usize + 1];
    while hi - lo > 1 {
        for v in memo.iter_mut() { *v = 0; }
        let mid = (hi + lo) / 2;
        for i in (mid as usize) .. (a_max as usize + 1) {
            let res = memo[i / 2] + memo[(i + 1) / 2];
            memo[i] = max(1, res);
        }
        let mut tot: u64 = 0;
        for i in 0 .. n {
            tot += memo[a[i] as usize] as u64;
        }
        if tot >= k {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    println!("{}", if lo == 0 { -1 } else { lo as i64 } );
}
