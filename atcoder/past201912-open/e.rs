#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, BufWriter, Write};

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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    let n: usize = get();
    let q: usize = get();
    let mut mat = vec![vec![false; n]; n];
    for _ in 0..q {
        let ty: i32 = get();
        match ty {
            1 => {
                let a = get::<usize>() - 1;
                let b = get::<usize>() - 1;
                mat[a][b] = true;
            }
            2 => {
                let a = get::<usize>() - 1;
                for i in 0..n {
                    if mat[i][a] {
                        mat[a][i] = true;
                    }
                }
            }
            3 => {
                let a = get::<usize>() - 1;
                let mut pool = vec![];
                for x in 0..n {
                    if x == a { continue; }
                    if mat[a][x] {
                        for i in 0..n {
                            if mat[x][i] {
                                pool.push(i);
                            }
                        }
                    }
                }
                for p in pool {
                    mat[a][p] = true;
                }
                mat[a][a] = false;
            }
            _ => unreachable!(),
        }
    }
    for i in 0..n {
        for j in 0..n {
            puts!("{}", if mat[i][j] { 'Y' } else { 'N' });
        }
        puts!("\n");
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
