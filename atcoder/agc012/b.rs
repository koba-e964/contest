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
    let mut edges = vec![Vec::new(); n];
    for _ in 0 .. m {
        let a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        edges[a].push(b);
        edges[b].push(a);
    }
    let q = get();
    let mut vdc = Vec::new();
    for _ in 0 .. q {
        let v = get::<usize>() - 1;
        let d: usize = get();
        let c: usize = get();
        vdc.push((v, d, c));
    }
    vdc.reverse();
    let mut ans = vec![0; n];
    let mut done = vec![0; n]; // offset 1
    for (v, d, c) in vdc {
        let mut que = VecDeque::new();
        que.push_back((v, d));
        while let Some((w, rem)) = que.pop_front() {
            if done[w] >= rem + 1 {
                continue;
            }
            if ans[w] == 0 {
                ans[w] = c;
            }
            done[w] = rem + 1;
            if rem > 0 {
                for &x in edges[w].iter() {
                    que.push_back((x, rem - 1));
                }
            }
        }
    }
    for v in ans {
        println!("{}", v);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
