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

const MOD: i64 = 998_244_353;

fn main() {
    let n: usize = get();
    let m: usize = get();
    let k: usize = get();
    if k == 1 {
        let mut tot = 0;
        for i in 1..std::cmp::min(n, m) + 1 {
            if m % i == 0 {
                tot += 1;
            }
        }
        for i in 1..n {
            tot = tot * i as i64 % MOD;
        }
        println!("{}", tot);
        return;
    }
    let mut tot = 0;
    for i in 2..n + 1 {
        if m % i != 0 {
            tot += 1;
        }
    }
    for i in 1..n - 1 {
        tot = tot * i as i64 % MOD;
    }
    println!("{}", tot);
}
