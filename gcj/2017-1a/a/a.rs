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
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn align(s: &mut [char]) -> bool {
    let all_question = s.iter().all(|&c| c == '?');
    if all_question {
        return false;
    }
    let n = s.len();
    let mut c = '\0';
    for i in 0 .. n {
        if c != '\0' && s[i] == '?' {
            s[i] = c;
        }
        if s[i] != '?' {
            c = s[i];
        }
    }
    for i in (0 .. n).rev() {
        if c != '\0' && s[i] == '?' {
            s[i] = c;
        }
        if s[i] != '?' {
            c = s[i];
        }
    }
    true
}

fn solve() {
    let t: usize = get();
    for case_nr in 1 .. t + 1 {
        let r: usize = get();
        let c: usize = get();
        let mut s: Vec<Vec<char>>
            = (0 .. r).map(|_| get_word().chars().collect()).collect();
        // align row first
        let mut aligned = vec![false; r];
        for i in 0 .. r {
            let res = align(&mut s[i]);
            aligned[i] = res;
        }
        let mut cur: Option<Vec<char>> = None;
        for i in 0 .. r {
            if cur.is_some() && !aligned[i] {
                s[i] = cur.clone().unwrap();
            }
            if aligned[i] {
                cur = Some(s[i].clone());
            }
        }
        for i in (0 .. r).rev() {
            if cur.is_some() && !aligned[i] {
                s[i] = cur.clone().unwrap();
            }
            if aligned[i] {
                cur = Some(s[i].clone());
            }
        }
        println!("Case #{}:", case_nr);
        for v in s {
            println!("{}", v.into_iter().collect::<String>());
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
