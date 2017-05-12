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
    let p: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let a: Vec<usize> = (0 .. n).map(|_| get()).collect();
    let b: Vec<usize> = (0 .. n).map(|_| get()).collect();
    let m = get();
    let c: Vec<usize> = (0 .. m).map(|_| get()).collect();
    let mut pool = vec![vec![Vec::new(); 4]; 4];
    for i in 0 .. n {
        pool[a[i]][b[i]].push(p[i]);
    }
    for i in 0 .. 4 {
        for j in 0 .. 4 {
            pool[i][j].sort();
            pool[i][j].reverse();
        }
    }
    for round in 0 .. m {
        let col = c[round];
        const INF: i64 = 1 << 60; 
        let mut mini = (100, 100);
        let mut mi: i64 = INF;
        for i in 1 .. 4 {
            for j in 1 .. 4 {
                if i != col && j != col { continue; }
                if pool[i][j].len() > 0 {
                    let uv = pool[i][j][pool[i][j].len() - 1];
                    if mi > uv {
                        mi = uv;
                        mini = (i, j);
                    }
                }
            }
        }
        if mi == INF {
            print!("-1");
        } else {
            print!("{}", mi);
            pool[mini.0][mini.1].pop();
        }
        print!("{}", if round == m - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
