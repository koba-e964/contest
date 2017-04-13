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

fn solve() {
    let t = get();
    for case_nr in 0 .. t {
        let s = get_word();
        let k = get();
        let mut a: Vec<bool> = s.chars().map(|v| v == '-').collect();
        let mut cnt = 0;
        let n = a.len();
        for i in 0 .. n + 1 - k {
            if a[i] {
                // greedily flips i .. i + k
                for j in 0 .. k {
                    a[i + j] = !a[i + j];
                }
                cnt += 1;
            }
        }
        // check if all is done
        print!("Case #{}: ", case_nr + 1);
        if a.iter().any(|&v| v) {
            println!("IMPOSSIBLE");
        } else {
            println!("{}", cnt);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
