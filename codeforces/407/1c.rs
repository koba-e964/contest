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
    let k = get();
    const W: usize = 1001;
    let mut has = vec![false; W];
    for _ in 0 .. k {
        let u: usize = get();
        has[u] = true;
    }
    const INF: i64 = 1 << 50;
    const BIAS: usize = W - 1;
    let mut dist = [INF; 2 * BIAS + 1];
    let mut que = VecDeque::new();
    que.push_back((BIAS, 0));
    while let Some((pt, d)) = que.pop_front() {
        if d >= dist[pt] {
            continue;
        }
        if d > 0 {
            dist[pt] = d;
        }
        for i in 0 .. W {
            if pt + i >= n {
                if has[i] {
                    let nxt = pt + i - n;
                    if nxt < 2 * BIAS + 1 {
                        que.push_back((nxt, d + 1));
                    }
                }
            }
        }
    }
    println!("{}", if dist[BIAS] == INF { -1 } else { dist[BIAS] });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
