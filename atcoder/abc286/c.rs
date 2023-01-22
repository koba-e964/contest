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
    let a: i64 = get();
    let b: i64 = get();
    let s: Vec<_> = get_word().chars().collect();
    let mut ans = b * n as i64;
    for i in 0..n {
        let mut x = s[i..].to_vec();
        x.extend_from_slice(&s[..i]);
        let mut c = 0;
        for j in 0..n / 2 {
            if x[j] != x[n - 1 - j] {
                c += b;
            }
        }
        ans = std::cmp::min(ans, c + a * i as i64);
    }
    println!("{}", ans);
}
