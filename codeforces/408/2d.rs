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

// This solution is implemented after the author read the editorial
fn solve() {
    let n = get();
    let k = get();
    let _d: usize = get();
    let p: Vec<usize> = (0 .. k).map(|_| get::<usize>() - 1).collect();
    let mut edges = vec![Vec::new(); n];
    for i in 1 .. n {
        let u = get::<usize>() - 1;
        let v = get::<usize>() - 1;
        edges[u].push((v, i));
        edges[v].push((u, i));
    }
    // bfs solution
    const INF: i32 = 1 << 29;
    let mut dist = vec![INF; n];
    let mut cutoff = HashSet::new();
    let mut que = VecDeque::new();
    for v in p {
        que.push_back((0, v, n, n));
    }
    while let Some((d, v, prev, eidx)) = que.pop_front() {
        if dist[v] <= d {
            if eidx < n {
                cutoff.insert(eidx);
            }
            continue;
        }
        dist[v] = d;
        for &(w, idx) in edges[v].iter() {
            if prev == w { continue; }
            que.push_back((d + 1, w, v, idx));
        }
    }
    let cutoff: Vec<_> = cutoff.into_iter().collect();
    println!("{}", cutoff.len());
    for i in 0 .. cutoff.len() {
        print!("{}{}", cutoff[i],
               if i == cutoff.len() - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
