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

fn solve() {
    let n: usize = get();
    let q: usize = get();
    let mut prow: Vec<_> = (0..n).collect();
    let mut pcol: Vec<_> = (0..n).collect();
    let mut flip = false;
    for _ in 0..q {
        let ty: i32 = get();
        match ty {
            1 => {
                let a = get::<usize>() - 1;
                let b = get::<usize>() - 1;
                if flip {
                    pcol.swap(a, b);
                } else {
                    prow.swap(a, b);
                }
            }
            2 => {
                let a = get::<usize>() - 1;
                let b = get::<usize>() - 1;
                if flip {
                    prow.swap(a, b);
                } else {
                    pcol.swap(a, b);
                }
            }
            3 => flip = !flip,
            4 => {
                let a = get::<usize>() - 1;
                let b = get::<usize>() - 1;
                let (i, j) = if flip {
                    (prow[b], pcol[a])
                } else {
                    (prow[a], pcol[b])
                };
                println!("{}", n * i + j);
            }
            _ => panic!(),
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
