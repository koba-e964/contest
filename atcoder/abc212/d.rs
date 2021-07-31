use std::cmp::*;
use std::collections::*;
use std::io::Read;

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

fn main() {
    let q: usize = get();
    let mut que = BinaryHeap::new();
    let mut del = 0;
    for _ in 0..q {
        let k: i32 = get();
        match k {
            1 => {
                let x: i64 = get();
                que.push(Reverse(x - del));
            }
            2 => {
                let x: i64 = get();
                del += x;
            }
            3 => {
                let v = que.pop().unwrap().0;
                println!("{}", v + del);
            }
            _ => panic!(),
        }
    }
}
