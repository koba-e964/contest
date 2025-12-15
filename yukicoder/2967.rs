use std::collections::*;
use std::cmp::*;
use std::io::Read;

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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn ask(i: usize, j: usize) -> Option<usize> {
    println!("1 {} {}", i + 1, j + 1);
    let x: i32 = get();
    if x == -1 {
        None
    } else {
        Some(x as usize - 1)
    }
}

// https://yukicoder.me/problems/no/2967 (3.5)
fn main() {
    let n: usize = get();
    let mut to = vec![None; n];
    let mut g = vec![vec![]; n];
    for i in 0..n {
        to[i] = ask(i, i);
        if let Some(v) = to[i] {
            g[v].push(i);
        }
    }
    const INF: i32 = 1 << 28;
    let mut dist = vec![INF; n];
    let mut que = VecDeque::new();
    for i in 0..n {
        if to[i].is_none() {
            que.push_back((0, i));
        }
    }
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d {
            continue;
        }
        dist[v] = d;
        for &w in &g[v] {
            que.push_back((d + 1, w));
        }
    }
    let mut ma = (0, 0);
    for i in 0..n {
        if dist[i] < INF {
            ma = max(ma, (dist[i], i));
        }
    }
    let one = ma.1;
    let two = to[one].unwrap();
    let mut ans = vec![0; n];
    ans[one] = 1;
    ans[two] = 2;
    let mut cur = two;
    for i in 2..n {
        let x = ask(one, cur).unwrap();
        cur = x;
        ans[x] = i + 1;
    }
    print!("2");
    for i in 0..n {
        print!(" {}", ans[i]);
    }
    println!();
}
