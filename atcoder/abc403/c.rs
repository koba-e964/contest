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
    let _m: usize = get();
    let q: usize = get();
    let mut indiv = vec![HashSet::new(); n];
    let mut all = vec![false; n];
    for _ in 0..q {
        let ty: i32 = get();
        let x = get::<usize>() - 1;
        if ty == 1 {
            let y = get::<usize>() - 1;
            indiv[x].insert(y);
        } else if ty == 2 {
            all[x] = true;
        } else {
            let y = get::<usize>() - 1;
            if all[x] || indiv[x].contains(&y) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
