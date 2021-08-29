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

fn ask(kind: char, x: usize, y: usize) -> i64 {
    match kind {
        '|' => println!("or {} {}", x + 1, y + 1),
        '&' => println!("and {} {}", x + 1, y + 1),
        _ => panic!(),
    };
    let x: i64 = get();
    x
}

fn solve() {
    // Hopefully not adaptive
    let n: usize = get();
    let k: usize = get();
    let mut xor = vec![0; n];
    let mut and = vec![0; n];
    let mut least = 0;
    let mut cnt = vec![[0; 2]; 30];
    for i in 1..n {
        let a = ask('&', 0, i);
        let b = ask('|', 0, i);
        xor[i] = a ^ b;
        and[i] = a;
        least |= a;
        for j in 0..30 {
            let u = ((xor[i] >> j) & 1) as usize;
            cnt[j][u] += 1;
        }
    }
    let res = ask('|', 1, 2);
    for i in 0..30 {
        if cnt[i][0] == 0 && cnt[i][1] > 0 {
            least |= (1 << i) - (res & 1 << i);
        }
    }
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = least ^ xor[i];
    }
    a.sort();
    // eprintln!("xor = {:?}, least = {}, a = {:?}", xor, least, a);
    println!("finish {}", a[k - 1]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
