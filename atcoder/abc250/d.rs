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

// Finds all primes <= lim.
// Verified: https://atcoder.jp/contests/abc195/submissions/22273725
fn primes(lim: usize) -> (Vec<bool>, Vec<usize>) {
    if lim <= 1 {
        return (vec![false; lim + 1], vec![]);
    }
    let mut pr = vec![true; lim + 1];
    pr[0] = false;
    pr[1] = false;
    for i in 2..=lim {
        if !pr[i] {
            continue;
        }
        for j in 2..=lim / i {
            pr[i * j] = false;
        }
    }
    (pr.clone(), (2..=lim).filter(|&i| pr[i]).collect())
}

fn main() {
    let n: i64 = get();
    const W: usize = 1_000_000;
    let (pr, ps) = primes(W);
    let mut acc = vec![0i64; W + 1];
    for i in 1..W + 1 {
        acc[i] = acc[i - 1] + if pr[i] { 1 } else { 0 };
    }
    let mut ans = 0i64;
    for p in ps {
        let pp = p as i64;
        let lim = n / (pp * pp * pp);
        let lim = min(lim, pp - 1);
        ans += acc[lim as usize];
    }
    println!("{}", ans);
}
