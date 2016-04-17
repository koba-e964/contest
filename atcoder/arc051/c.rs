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


const MD: i64 = 1_000_000_007;
fn powmod(a: i64, b: i64) -> i64 {
    let mut e = b;
    let mut sum = 1;
    let mut cur = a % MD;
    while e > 0 {
        if e % 2 == 1 {
            sum = sum * cur % MD;
        }
        cur = cur * cur % MD;
        e /= 2;
    }
    return sum;
}

fn main() {
    let n: usize = get();
    let a: i64 = get();
    let b: i64 = get();
    let al: f64 = (a as f64).log(2.0);
    let mut v: Vec<i64> = vec![0; n];
    let mut lg: Vec<f64> = vec![0.0; n];
    let mut mulcnt: Vec<i64> = vec![0; n];
    for i in 0 .. n {
        v[i] = get::<i64>();
        lg[i] = (v[i] as f64).log(2.0);
    }
    if a == 1 {
        v.sort();
        for i in 0 .. n {
            println!("{}", v[i]);
        }
        return;
    }
    // a >= 2
    // multiply until difference becomes small
    let mut cnt = 0;
    while cnt < b {
        let mut ma = lg[0];
        let mut mi = lg[0];
        let mut mini = 0;
        for i in 1 .. n {
            if ma < lg[i] {
                ma = lg[i];
            }
            if mi > lg[i] {
                mi = lg[i];
                mini = i;
            }
        }
        if ma - mi < al {
            break;
        }
        lg[mini] += al;
        mulcnt[mini] += 1;
        cnt += 1;
    }
    let r = (b - cnt) / (n as i64);
    for i in 0 .. n {
        mulcnt[i] += r;
        cnt += r;
    }
    while cnt < b {
        let mut mi = lg[0];
        let mut mini = 0;
        for i in 1 .. n {
            if mi > lg[i] {
                mi = lg[i];
                mini = i;
            }
        }
        lg[mini] += al;
        mulcnt[mini] += 1;
        cnt += 1;
    }
    let mut idx = vec![0; n];
    for i in 0 .. n {
        idx[i] = i;
    }
    idx.sort_by(|&i, &j| lg[i].partial_cmp(&lg[j]).unwrap());
    for i in 0 .. n {
        println!("{}", v[idx[i]] * powmod(a, mulcnt[idx[i]]) % MD);
    }
}
