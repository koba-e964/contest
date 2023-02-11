use std::collections::*;
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

fn main() {
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let m: usize = get();
        let c: Vec<i32> = (0..n).map(|_| get()).collect();
        let mut g = vec![vec![]; n];
        for _ in 0..m {
            let a = get::<usize>() - 1;
            let b = get::<usize>() - 1;
            g[a].push(b);
            g[b].push(a);
        }
        const INF: i32 = 1 << 28;
        let mut dist = vec![INF; n * n];
        let mut que = VecDeque::new();
        que.push_back((0, 0, n - 1));
        while let Some((d, x, y)) = que.pop_front() {
            if dist[x * n + y] <= d { continue; }
            dist[x * n + y] = d;
            for &w1 in &g[x] {
                for &w2 in &g[y] {
                    if c[w1] != c[w2] {
                        que.push_back((d + 1, w1, w2));
                    }
                }
            }
        }
        let ans = dist[(n - 1) * n];
        println!("{}", if ans >= INF { -1 } else { ans });
    }
}
