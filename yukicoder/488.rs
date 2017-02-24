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
    let n = get();
    let m = get();
    let mut mat = vec![vec![false; n]; n];
    for _ in 0 .. m {
        let a: usize = get();
        let b: usize = get();
        mat[a][b] = true;
        mat[b][a] = true;
    }
    let mut cnt: i32 = 0;
    for i in 0 .. n {
        for j in i + 1 .. n {
            for k in j + 1 .. n {
                for l in k + 1 .. n {
                    let sl = [i, j, k, l];
                    let mut flag: i32 = 0;
                    for d in 0 .. 4 {
                        for e in 0 .. 4 {
                            if mat[sl[d]][sl[e]]  {
                                flag |= 1 << 4 * d + e;
                            }
                        }
                    }
                    if flag == 0x5a5a // 0-1-2-3
                        || flag == 0x6996 // 0-1-3-2
                        || flag == 0x33cc // 0-2-1-3
                    {
                        cnt += 1;
                    }
                }
            }
        }
    }
    println!("{}", cnt);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
