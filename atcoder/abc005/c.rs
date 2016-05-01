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


const N: usize = 101;


fn calc(t: i32, a: &mut [usize], b: &[usize]) -> bool {
    let mut cur: usize = 0;
    for i in 0 .. N {
        let mut rem: usize = b[i];
        cur = max(cur as i32, (i as i32) - t) as usize;
        while cur <= i && rem > 0 {
            let diff = min(a[cur], rem);
            a[cur] -= diff;
            rem -= diff;
            if a[cur] == 0 {
                cur += 1;
            }
        }
        if rem > 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let t: i32 = get();
    let n: usize = get();
    let mut a: Vec<usize> = vec![0; N];
    for _ in 0 .. n {
        let u: usize = get();
        a[u] += 1;
    }
    let m: usize = get();
    let mut b: Vec<usize> = vec![0; N];
    for _ in 0 .. m {
        let u: usize = get();
        b[u] += 1;
    }
    println!("{}", if calc(t, &mut a, &b) { "yes" } else { "no" });
}
