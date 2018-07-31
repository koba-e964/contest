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

const INF: usize = 1 << 27;

fn solve() {
    let n: usize = get();
    let m: usize = get();
    let mut s = vec![Vec::new(); n];
    let mut t = vec![vec!['.'; m]; n];
    for i in 0 .. n {
        s[i] = get_word().chars().collect();
    }
    let mut vert = vec![vec![INF; m]; n];
    let mut hori = vec![vec![INF; m]; n];
    for i in 0 .. n {
        let mut cnt = 0; 
        for j in 0 .. m {
            if s[i][j] == '*' {
                cnt += 1;
            } else {
                cnt = 0;
            }
            vert[i][j] = min(vert[i][j], cnt);
        }
        let mut cnt = 0; 
        for j in (0 .. m).rev() {
            if s[i][j] == '*' {
                cnt += 1;
            } else {
                cnt = 0;
            }
            vert[i][j] = min(vert[i][j], cnt);
        }
    }
    for j in 0 .. m {
        let mut cnt = 0; 
        for i in 0 .. n {
            if s[i][j] == '*' {
                cnt += 1;
            } else {
                cnt = 0;
            }
            hori[i][j] = min(hori[i][j], cnt);
        }
        let mut cnt = 0; 
        for i in (0 .. n).rev() {
            if s[i][j] == '*' {
                cnt += 1;
            } else {
                cnt = 0;
            }
            hori[i][j] = min(hori[i][j], cnt);
        }
    }
    let mut ops = Vec::new();
    for i in 0 .. n {
        for j in 0 .. m {
            let k = max(1, min(vert[i][j], hori[i][j]));
            vert[i][j] = k - 1;
            hori[i][j] = k - 1;
            if k > 1 {
                ops.push((i, j, k - 1));
            }
        }
    }
    for i in 0 .. n {
        for j in 0 .. m - 1 {
            vert[i][j + 1] = max(vert[i][j + 1], max(1, vert[i][j]) - 1);
        }
        for j in (0 .. m - 1).rev() {
            vert[i][j] = max(vert[i][j], max(1, vert[i][j + 1]) - 1);
        }
    }
    for j in 0 .. m {
        for i in 0 .. n - 1 {
            hori[i + 1][j] = max(hori[i + 1][j], max(1, hori[i][j]) - 1);
        }
        for i in (0 .. n - 1).rev() {
            hori[i][j] = max(hori[i][j], max(1, hori[i + 1][j]) - 1);
        }
    }
    for i in 0 .. n {
        for j in 0 .. m {
            if vert[i][j] >= 1 {
                for k in j - 1 .. j + 2 {
                    t[i][k] = '*';
                }
            }
        }
    }
    for j in 0 .. m {
        for i in 0 .. n {
            if hori[i][j] >= 1 {
                for k in i - 1 .. i + 2 {
                    t[k][j] = '*';
                }
            }
        }
    }
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    if s == t {
        writeln!(out, "{}", ops.len()).unwrap();
        for (a, b, c) in ops {
            writeln!(out, "{} {} {}", a + 1, b + 1, c).unwrap();
        }
    } else {
        writeln!(out, "-1").unwrap();
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
