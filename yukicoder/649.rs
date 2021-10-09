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

// Tags: data-structure, set, two-sets
fn main() {
    let q: usize = get();
    let k: usize = get();
    let mut fst = BTreeSet::new();
    let mut snd = BTreeSet::new();
    for i in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let v: i64 = get();
            fst.insert((v, i));
            if fst.len() > k {
                let &x = fst.iter().rev().next().unwrap();
                fst.remove(&x);
                snd.insert(x);
            }
        } else {
            if fst.len() < k {
                println!("-1");
            } else {
                let &x = fst.iter().rev().next().unwrap();
                println!("{}", x.0);
                fst.remove(&x);
                if let Some(&y) = snd.iter().next() {
                    fst.insert(y);
                    snd.remove(&y);
                }
            }
        }
    }
}
