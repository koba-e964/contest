#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, BufWriter, Write};

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

// Tags: knapsack dp optimal-strategy
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let k: usize = get();
        let mut ba: Vec<_> = (0..n).map(|i| {
            let a: i32 = get();
            let b: i32 = get();
            (b, a, i)
        }).collect();
        ba.sort();
        const INF: i32 = 1 << 28;
        let mut dp = vec![vec![(-INF, 0, 0); k + 1]; n + 1];
        dp[0][0] = (0, 0, n);
        for i in 0..n {
            let (b, a, idx) = ba[i];
            for j in 0..k + 1 {
                dp[i + 1][j] = max(dp[i + 1][j], (dp[i][j].0, j, n));
                if j < k {
                    let val = a - b * (k - j - 1) as i32;
                    dp[i + 1][j + 1] = max(dp[i + 1][j + 1],
                                           (dp[i][j].0 + val, j, idx));
                }
            }
        }
        let mut cur = (n, k);
        let mut picked = vec![];
        let mut unused = vec![true; n];
        while cur.0 > 0 {
            let (i, j) = cur;
            let (_, pre, idx) = dp[i][j];
            if j != pre {
                picked.push(idx);
                cur.1 = pre;
                unused[idx] = false;
            }
            cur.0 -= 1;
        }
        picked.reverse();
        let mut ops = vec![];
        assert_eq!(picked.len(), k);
        for i in 0..k - 1 {
            ops.push(picked[i] as i32 + 1);
        }
        for i in 0..n {
            if unused[i] {
                ops.push(i as i32 + 1);
                ops.push(-(i as i32) - 1);
            }
        }
        ops.push(picked[k - 1] as i32 + 1);
        puts!("{}\n", ops.len());
        for i in 0..ops.len() {
            puts!("{}{}", ops[i], if i + 1 == ops.len() { "\n" } else { " " });
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
