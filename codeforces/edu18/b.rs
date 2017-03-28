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
    let k = get();
    let a: Vec<usize> = (0 .. k).map(|_| get()).collect();
    let mut circle = vec![0; n];
    for i in 0 .. n {
        circle[i] = i + 1;
    }
    let mut output = Vec::new();
    for v in a {
        // Get the v-th
        let v = v % circle.len();
        output.push(circle[v]);
        let mut nxt = circle[v + 1 ..].to_vec();
        nxt.extend_from_slice(&circle[.. v]);
        circle = nxt;
    }
    for i in 0 .. output.len() {
        print!("{}{}", output[i],
               if i == output.len() - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
