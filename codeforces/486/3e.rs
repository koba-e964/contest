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

const INF: i32 = 1 << 29;

fn calc(n: &[char], pat: &[char]) -> i32 {
    let len = n.len();
    let mut mi = INF;
    if len == 2 {
        for j in 0 .. len {
            if pat[0] != n[j] { continue; }
            for k in 0 .. len {
                if j == k || pat[1] != n[k] { continue; }
                let mut cost = 0;
                let mut x = j as i32;
                let mut y = k as i32;
                cost += (y - len as i32 + 1).abs();
                if x > y { x -= 1; }
                cost += (x - len as i32 + 2).abs();
                eprintln!("{:?} ==> {}", (j, k), cost);
                mi = min(mi, cost);
            }
        }
        return mi;
    }
    for i in 0 .. len {
        if n[i] == '0' { continue; }
        for j in 0 .. len {
            if i == j || pat[0] != n[j] { continue; }
            for k in 0 .. len {
                if i == k || j == k || pat[1] != n[k] { continue; }
                let mut cost = 0;
                cost += i as i32;
                let mut x = if i < j { j } else { j + 1 } as i32;
                let mut y = if i < k { k } else { k + 1 } as i32;
                cost += (y - len as i32 + 1).abs();
                if x > y { x -= 1; }
                cost += (x - len as i32 + 2).abs();
                eprintln!("{:?} ==> {}", (i, j, k), cost);
                mi = min(mi, cost);
            }
        }
    }
    return mi;
}


fn solve() {
    let n: Vec<char> = get_word().chars().collect();
    let mut mi = INF;
    for pat in [['0', '0'], ['2', '5'], ['5', '0'], ['7', '5']].iter() {
        mi = min(mi, calc(&n, pat));
    }
    println!("{}", if mi == INF { -1 } else { mi }); 
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
