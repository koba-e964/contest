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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    let n: usize = get();
    let q: usize = get();
    let mut bx = vec![vec![]; n];
    let mut inv = vec![BTreeSet::new(); 200_100];
    for _ in 0..q {
        let ty: i32 = get();
        let i: usize = get();
        if ty == 1 {
            let j: usize = get();
            bx[j - 1].push(i);
            inv[i - 1].insert(j);
            continue;
        }
        if ty == 2 {
            bx[i - 1].sort_unstable();
            for &v in &bx[i - 1] {
                print!("{} ", v);
            }
            println!();
            continue;
        }
        for &v in inv[i - 1].iter() {
            print!("{} ", v);
        }
        println!();
    }
}
