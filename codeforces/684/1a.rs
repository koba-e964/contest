#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};

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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let m: usize = get();
        let mut s = vec![vec![]; n];
        for i in 0..n {
            s[i] = get_word().bytes().map(|x| x == b'1').collect();
        }
        let mut ops = vec![];
        for i in 0..n - 2 {
            for j in 0..m {
                if s[i][j] {
                    let nxt = if j > 0 { j - 1 } else { 1 };
                    ops.push(vec![i, j, i + 1, j, i + 1, nxt]);
                    s[i][j] ^= true;
                    s[i + 1][j] ^= true;
                    s[i + 1][nxt] ^= true;
                }
            }
        }
        for j in 0..m - 2 {
            let mut op = vec![];
            let i = n - 2;
            match (s[n - 2][j] as i32, s[n - 1][j] as i32) {
                (0, 0) => (),
                (0, 1) => op = vec![i + 1, j, i, j + 1, i + 1, j + 1],
                (1, 0) => op = vec![i, j, i, j + 1, i + 1, j + 1],
                (1, 1) => op = vec![i, j, i + 1, j, i + 1, j + 1],
                _ => panic!(),
            };
            if !op.is_empty() {
                for i in 0..3 {
                    s[op[2 * i]][op[2 * i + 1]] ^= true;
                }
                ops.push(op);
            }
        }
        let pat = [7, 11, 13, 14];
        let mut des = 0;
        for i in 0..2 {
            for j in 0..2 {
                if s[n - 2 + i][m - 2 + j] {
                    des |= 1 << (2 * i + j);
                }
            }
        }
        let mut ans = 0;
        for bits in 0..16 {
            let mut x = 0;
            for i in 0..4 {
                if (bits & 1 << i) != 0 {
                    x ^= pat[i];
                }
            }
            if x == des {
                ans = bits;
                break;
            }
        }
        for i in 0..4 {
            if (ans & 1 << i) == 0 {
                continue;
            }
            let mut op = vec![];
            for j in 0..4 {
                if (pat[i] & 1 << j) != 0 {
                    let x = n - 2 + j / 2;
                    let y = m - 2 + (j % 2);
                    op.push(x);
                    op.push(y);
                }
            }
            for i in 0..3 {
                s[op[2 * i]][op[2 * i + 1]] ^= true;
            }
            ops.push(op);
        }
        puts!("{}\n", ops.len());
        for op in ops {
            for j in 0..6 {
                puts!("{}{}", op[j] + 1, if j + 1 == 6 { "\n" } else { " " });
            }
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
