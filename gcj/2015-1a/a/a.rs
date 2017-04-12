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
    let t: usize = get();
    for case_nr in 1 .. t + 1 {
    	let n = get();
	let a: Vec<i32> = (0 .. n).map(|_| get()).collect();
	let mut fst = 0;
	let mut snd = 0;
	let mut grad = 0;
	for i in 0 .. n - 1 {
	    fst += max(a[i] - a[i + 1], 0);
	    grad = max(grad, a[i] - a[i + 1]);
	}
	for i in 0 .. n - 1 {
	    snd += min(a[i], grad);
	}
	println!("Case #{}: {} {}", case_nr, fst, snd);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
