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


const INF: i64 = 1 << 50;

fn solve() {
    loop {
        let w: usize = get();
        let h: usize = get();
        if w == 0 && h == 0 { break; }
        let mut g = vec![Vec::new(); w * h];
        for i in 0 .. h {
            for j in 0 .. w - 1 {
                let a: i32 = get();
                if a == 0 {
                    g[i * w + j].push(i * w + j + 1);
                    g[i * w + j + 1].push(i * w + j);
                }
            }
            if i + 1 < h {
                for j in 0 .. w {
                    let a: i32 = get();
                    if a == 0 {
                        g[i * w + j].push(i * w + j + w);
                        g[i * w + j + w].push(i * w + j);
                    }
                }
            }
        }
        let mut dist = vec![INF; w * h];
        let mut que = VecDeque::new();
        que.push_back((0, 0));
        while let Some((v, d)) = que.pop_front() {
            if dist[v] <= d { continue; }
            dist[v] = d;
            for &w in &g[v] {
                que.push_back((w, d + 1));
            }
        }
        println!("{}", if dist[w * h - 1] >= INF { 0 } else { dist[w * h - 1] + 1 });
    }
}
    
fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
