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
    let n: usize = get();
    let m: usize = get();
    let q: usize = get();
    let mut s = vec![Vec::new(); n];
    for i in 0 .. n {
        s[i] = get_word().chars().map(|x| (x as u8 - 0x30) as i32).collect();
    }
    let mut acc = vec![vec![0; m + 1]; n + 1];
    let mut link1 = vec![vec![0; m + 1]; n + 1];
    let mut link2 = vec![vec![0; m + 1]; n + 1];
    for i in 0 .. n {
        for j in 0 .. m {
            acc[i + 1][j + 1] = acc[i + 1][j] + acc[i][j + 1] - acc[i][j]
                + s[i][j];
            link1[i + 1][j + 1] = link1[i + 1][j] + link1[i][j + 1]
                - link1[i][j];
            link2[i + 1][j + 1] = link2[i + 1][j] + link2[i][j + 1]
                - link2[i][j];
            if j < m - 1 && s[i][j] == 1 && s[i][j + 1] == 1 {
                link1[i + 1][j + 1] += 1;
            }
            if i < n - 1 && s[i][j] == 1 && s[i + 1][j] == 1 {
                link2[i + 1][j + 1] += 1;
            }
        }
    }
    for _ in 0 .. q {
        let x1: usize = get::<usize>() - 1;
        let y1: usize = get::<usize>() - 1;
        let x2: usize = get();
        let y2: usize = get();
        let cell = acc[x2][y2] - acc[x1][y2] - acc[x2][y1] + acc[x1][y1];
        let l1 = link1[x2][y2 - 1] - link1[x1][y2 - 1] - link1[x2][y1]
            + link1[x1][y1];
        let l2 = link2[x2 - 1][y2] - link2[x1][y2] - link2[x2 - 1][y1]
            + link2[x1][y1];
        println!("{}", cell - l1 - l2);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
