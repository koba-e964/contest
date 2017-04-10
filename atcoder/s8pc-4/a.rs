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

fn comp(s: &[char], t: &[char], o: Ordering) -> bool {
    let mut pos = 0;
    let lim = match o {
        Ordering::Less => 'a',
        Ordering::Greater => 'z',
        _ => panic!(),
    };
    loop {
        if pos >= s.len() && pos >= t.len() {
            return true;
        }
        if pos >= t.len() {
            return o == Ordering::Greater;
        }
        if pos >= s.len() {
            return o == Ordering::Less;
        }
        if s[pos] == '?' {
            if t[pos] != lim {
                return true;
            }
        }
        if s[pos] != t[pos] {
            return s[pos].cmp(&t[pos]) == o;
        }
        pos += 1;
    }
}

fn solve() {
    let n = get();
    let s: Vec<Vec<_>> = (0 .. n).map(|_| get_word().chars().collect())
        .collect();
    let t: Vec<_> = get_word().chars().collect();
    let mut le = 0;
    let mut ge = 0;
    for i in 0 .. n {
        if comp(&s[i], &t, Ordering::Less) {
            le += 1;
        }
        if comp(&s[i], &t, Ordering::Greater) {
            ge += 1;
        }
    }
    for i in n - ge .. le + 1 {
        print!("{}{}", i + 1, if i == le { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
