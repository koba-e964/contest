#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
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
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn filter(a: String, x: &str) -> String {
    if a.len() != x.len() {
        return a;
    }
    let a: Vec<_> = a.chars().collect();
    let x: Vec<_> = x.chars().collect();
    let n = a.len();
    let mut ok = true;
    for i in 0..n {
        if a[i] != x[i] && x[i] != '*' {
            ok = false;
        }
    }
    if ok {
        return vec!['*'; n].into_iter().collect();
    }
    a.into_iter().collect()
}

fn main() {
    let line = getline();
    let s: Vec<_> = line.trim().split(" ").collect();
    let n: usize = get();
    let mut fil = vec![];
    for _ in 0..n {
        fil.push(get_word());
    }
    let l = s.len();
    for i in 0..l {
        let mut a = s[i].to_owned();
        for j in 0..n {
            a = filter(a, &fil[j]);
        }
        print!("{}{}", a, if i + 1 == l { "\n" } else { " " });
    }
}
