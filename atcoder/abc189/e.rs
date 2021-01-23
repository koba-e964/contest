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

fn solve() {
    let n: usize = get();
    let xy: Vec<(i64, i64)> = (0..n).map(|_| {
        let x = get();
        let y = get();
        (x, y)
    }).collect();
    let m: usize = get();
    let mut ops = vec![];
    for _ in 0..m {
        let ty: i32 = get();
        let op;
        if ty >= 3 {
            let p: i64 = get();
            op = (ty, p);
        } else {
            op = (ty, 0);
        }
        ops.push(op);
    }
    let q: usize = get();
    let mut qs = vec![vec![]; m + 1];
    for idx in 0..q {
        let a: usize = get();
        let b: usize = get::<usize>() - 1;
        qs[a].push((b, idx));
    }
    let mut ans = vec![(-1, -1); q];
    let mut transf = [[0; 3]; 3];
    for i in 0..3 {
        transf[i][i] = 1;
    }
    for i in 0..m + 1 {
        for &(b, idx) in &qs[i] {
            let mut res = [0; 3];
            let (x, y) = xy[b];
            let orig = [x, y, 1];
            for j in 0..3 {
                for k in 0..3 {
                    res[j] += transf[j][k] * orig[k];
                }
            }
            ans[idx] = (res[0], res[1]);
        }
        if i < m {
            let (ty, p) = ops[i];
            let mut left = [[0; 3]; 3];
            if ty <= 2 {
                left[0][1] = if ty == 1 { 1 } else { -1 };
                left[1][0] = -left[0][1];
                left[2][2] = 1;
            } else {
                for j in 0..3 {
                    left[j][j] = 1;
                }
                if ty == 3 {
                    left[0][0] = -1;
                    left[0][2] = 2 * p;
                } else {
                    left[1][1] = -1;
                    left[1][2] = 2 * p;
                }
            }
            let mut after = [[0; 3]; 3];
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        after[j][l] += left[j][k] * transf[k][l];
                    }
                }
            }
            transf = after;
        }
    }
    for &(x, y) in &ans {
        println!("{} {}", x, y);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
