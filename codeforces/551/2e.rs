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

fn ask(x1: usize, x2: usize, y1: usize, y2: usize) -> i32 {
    println!("? {} {} {} {}", x1 + 1, y1 + 1, x2, y2);
    let x = get();
    if x == -1 {
        panic!();
    }
    x
}

fn answer(x1: usize, y1: usize, x2: usize, y2: usize) {
    println!("! {} {} {} {}", x1 + 1, y1 + 1, x2 + 1, y2 + 1);
}

fn solve() {
    let n: usize = get();
    let mut row = vec![0; n];
    let mut col = vec![0; n];
    let mut cur = 0;
    for i in 0..n - 1 {
        row[i] = ask(i, i + 1, 0, n) % 2;
        cur ^= row[i];
    }
    row[n - 1] = cur;
    cur = 0;
    for i in 0..n - 1 {
        col[i] = ask(0, n, i, i + 1) % 2;
        cur ^= col[i];
    }
    col[n - 1] = cur;
    let rs: Vec<usize> = (0..n).filter(|&i| row[i] == 1).collect();
    let cs: Vec<usize> = (0..n).filter(|&i| col[i] == 1).collect();
    assert!(rs.len() == 0 || rs.len() == 2);
    assert!(cs.len() == 0 || cs.len() == 2);
    if rs.is_empty() {
        let y1 = cs[0];
        let y2 = cs[1];
        let mut pass = n;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if ask(0, mid, y1, y1 + 1) % 2 == 1 {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        answer(fail, y1, fail, y2);
        return;
    }
    if cs.is_empty() {
        let x1 = rs[0];
        let x2 = rs[1];
        let mut pass = n;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if ask(x1, x1 + 1, 0, mid) % 2 == 1 {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        answer(x1, fail, x2, fail);
        return;
    }
    let (x1, y1) = (rs[0], cs[0]);
    let (x2, y2) = (rs[1], cs[1]);
    if ask(x1, x1 + 1, y1, y1 + 1) == 1 {
        answer(x1, y1, x2, y2);
    } else {
        answer(x1, y2, x2, y1);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
