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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    let n: usize = get();
    const W: usize = 2010;
    let mut f = vec![0; W];
    for _ in 0..n {
        let x: usize = get();
        f[x] += 1;
    }
    let mut mi = (n + 1, 0);
    for i in 0..W {
        if f[i] > 0 {
            mi = min(mi, (f[i], i));
        }
    }
    println!("{}", mi.1);
}
