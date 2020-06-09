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

trait Int {
    fn ask(&self, a: &[usize]) -> u64;
    fn ans(&self, a: &[u64]);
}

struct Real;

impl Int for Real {
    fn ask(&self, a: &[usize]) -> u64 {
        if a.len() == 0 {
            return 0;
        }
        print!("? {}", a.len());
        for &a in a {
            print!(" {}", a + 1);
        }
        println!();
        get()
    }
    fn ans(&self, a: &[u64]) {
        print!("!");
        for &a in a {
            print!(" {}", a);
        }
        println!();
    }
}

// The author read the editorial.
// Tags: antichain bitmask construction combination
fn solve() {
    let n: usize = get();
    let mut masks = vec![];
    for bits in 0usize..1 << 13 {
        if bits.count_ones() == 6 {
            masks.push(bits);
        }
    }
    let mut qs = vec![vec![]; 13];
    for i in 0..13 {
        for j in 0..n {
            if (masks[j] & 1 << i) != 0 {
                qs[i].push(j);
            }
        }
    }
    let mut b = vec![0; 13];
    let int = Real;
    for i in 0..13 {
        b[i] = int.ask(&qs[i]);
    }
    let mut a = vec![0; n];
    for i in 0..n {
        for j in 0..13 {
            if (masks[i] & 1 << j) == 0 {
                a[i] |= b[j];
            }
        }
    }
    int.ans(&a);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
