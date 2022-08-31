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

fn solve(a: &[usize], k: usize) -> bool {
    let mut hm = HashMap::new();
    for &a in a {
        *hm.entry(a).or_insert(0) += 1;
    }
    hm.values().all(|&v| v <= 2) && a.len() <= 2 * k
}

fn main() {
    let t: usize = get();
    for case_nr in 1..t + 1 {
        let n: usize = get();
        let k: usize = get();
        let a: Vec<usize> = (0..n).map(|_| get()).collect();
        println!("Case #{}: {}", case_nr, if solve(&a, k) { "YES" } else { "NO" });
    }
}
