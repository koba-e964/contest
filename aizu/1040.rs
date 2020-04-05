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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    loop {
        let h: usize = get();
        let w: usize = get();
        if h + w == 0 { break; }
        let a = (0..h).map(|_| {
            (0..w).map(|_| get::<i32>()).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        let mut term = vec![];
        for i in 0..h {
            for j in 0..w {
                let v = i * w + j;
                if a[i][j] == 1 {
                    term.push(v);
                }
            }
        }
        let k = term.len();
        const INF: i32 = 1 << 30;
        let n = h * w;
        let mut g = vec![vec![]; n];
        for i in 0..h {
            for j in 0..w {
                let v = i * w + j;
                let dxy = [(0, 1), (1, 0)];
                for &(dx, dy) in &dxy {
                    let nx = i + dx;
                    let ny = j + dy;
                    if nx >= h || ny >= w { continue; }
                    let v2 = nx * w + ny;
                    g[v].push(v2);
                    g[v2].push(v);
                }
            }
        }
        let mut dp = vec![vec![INF; 1 << k]; n];
        for i in 0..k {
            dp[term[i]][1 << i] = 0;
        }
        // Steiner tree
        // Unnecessarily slow (O(4^k))
        for bits in 1..1 << k {
            for i in 0..n {
                for sub in 1..bits {
                    if (sub & bits) != sub { continue; }
                    dp[i][bits] = min(
                        dp[i][bits],
                        dp[i][sub] + dp[i][bits - sub]);
                }
            }
            let mut que = BinaryHeap::new();
            for i in 0..n {
                if dp[i][bits] < INF {
                    que.push((-dp[i][bits], i));
                    dp[i][bits] = INF;
                }
            }
            while let Some((d, v)) = que.pop() {
                let d = -d;
                if dp[v][bits] <= d { continue; }
                dp[v][bits] = d;
                for &w in &g[v] {
                    que.push((-(d + 1), w));
                }
            }
        }
        let mut ma = 0;
        for i in 0..n {
            ma = max(ma, (h * w - 1) as i32 - dp[i][(1 << k) - 1]);
        }
        println!("{}", ma);
    }
}
