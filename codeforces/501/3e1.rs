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
    let m: usize = get();
    let mut s = vec![Vec::new(); n];
    let mut t = vec![vec!['.'; m]; n];
    for i in 0 .. n {
        s[i] = get_word().chars().collect();
    }
    let mut ops = Vec::new();
    for i in 0 .. n {
        for j in 0 .. m {
            if s[i][j] == '.' { continue; }
            let lim = min(min(i, j), min(n - 1 - i, m - 1 - j));
            let mut dis = 0;
            while dis < lim {
                if s[i - dis - 1][j] == '*' && s[i + dis + 1][j] == '*'
                    && s[i][j - dis - 1] == '*' && s[i][j + dis + 1] == '*' {
                        dis += 1;
                    } else {
                        break;
                    }
            }
            if dis > 0 {
                ops.push((i, j, dis));
                for k in 0 .. dis {
                    t[i - k - 1][j] = '*';
                    t[i + k + 1][j] = '*';
                    t[i][j - k - 1] = '*';
                    t[i][j + k + 1] = '*';
                }
                t[i][j] = '*';
            }
        }
    }
    if s == t {
        println!("{}", ops.len());
        for (a, b, c) in ops {
            println!("{} {} {}", a + 1, b + 1, c);
        }
    } else {
        println!("-1");
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
