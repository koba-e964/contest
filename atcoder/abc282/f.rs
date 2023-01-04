use std::cmp::*;
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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    let n: usize = get();
    let m = 12 * n;
    println!("{}", m);
    for i in 0..12 {
        let len = 1 << i;
        for j in 0..n {
            println!("{} {}", j + 1, min(j + len, n));
        }
    }
    let q: usize = get();
    for _ in 0..q {
        let l: usize = get();
        let r: usize = get();
        let len = r - l + 1;
        let mut c = 0;
        while 2 << c < len {
            c += 1;
        }
        println!("{} {}", n * c + l, n * c + r + 1 - (1 << c));
    }
}
