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

const W: usize = 1000;
fn fill(board: &mut [[usize; W]; W], x: &mut i32, y: &mut i32, count: usize,
        dx: &mut i32, dy: &mut i32, acc: &mut usize) {
    for _ in 0 .. count {
        board[*x as usize][*y as usize] = *acc;
        *x += *dx;
        *y += *dy;
        *acc += 1;
    }
    let (nx, ny) = (-*dy, *dx);
    *dx = nx;
    *dy = ny;
}

const N: usize = 1000100;

fn prime_init() -> [bool; N] {
    let mut pr = [true; N];
    pr[0] = false;
    pr[1] = false;
    for i in 2 .. N {
        if !pr[i] { continue; }
        for j in 2 .. (N - 1) / i + 1 {
            pr[i * j] = false;
        }
    }
    pr
}

const INF: i32 = 1 << 28;

fn solve() {
    let mut board = [[0; W]; W];
    let mut x = W as i32 / 2;
    let mut y = W as i32 / 2 - 1;
    let mut dx = 1;
    let mut dy = 0;
    let mut count = 1;
    for i in 0 .. W  / 2 {
        fill(&mut board, &mut x, &mut y, 2 * i,
             &mut dx, &mut dy, &mut count);
        fill(&mut board, &mut x, &mut y, 2 * i + 1,
             &mut dx, &mut dy, &mut count);
        fill(&mut board, &mut x, &mut y, 2 * i + 1,
             &mut dx, &mut dy, &mut count);
        fill(&mut board, &mut x, &mut y, 2 * i + 2,
             &mut dx, &mut dy, &mut count);
    }
    let pr = prime_init();
    let mut dp = [[0; W]; W];
    loop {
        let a: usize = get();
        let b: usize = get();
        if a == 0 && b == 0 { break; }
        for i in 0 .. W {
            for e in dp[i].iter_mut() {
                *e = -INF;
            }
        }
        for i in 0 .. W {
            for j in 0 .. W {
                if board[i][j] == b {
                    dp[i][j] = if pr[b] { 1 } else { 0 };
                    continue;
                }
                let cur = if board[i][j] <= a && pr[board[i][j]] {
                    1
                } else {
                    0
                };
                let mut ma = -INF;
                if i > 0 {
                    if j > 0 { ma = max(ma, dp[i - 1][j - 1]); }
                    ma = max(ma, dp[i - 1][j]);
                    if j < W - 1 { ma = max(ma, dp[i - 1][j + 1]); }
                }
                dp[i][j] = ma + cur;
            }
        }
        let mut ma = (0, 0);
        for i in 0 .. W {
            for j in 0 .. W {
                if board[i][j] <= a && pr[board[i][j]] {
                    ma = max(ma, (dp[i][j], board[i][j]));
                }
            }
        }
        println!("{} {}", ma.0, ma.1);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
