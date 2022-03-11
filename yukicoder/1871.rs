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

fn is_prime(x: i64) -> bool {
    if x <= 1 { return false; }
    let mut p = 2;
    while p * p <= x {
        if x % p == 0 { return false; }
        p += 1;
    }
    true
}
fn main() {
    let mut n: i64 = get();
    if n == 0 {
        println!("-1");
        return;
    }
    let mut ans = vec![];
    while n >= 2 {
        let mut c = 1;
        while c <= n {
            c *= 2;
        }
        c /= 2;
        let mut r = c - 1;
        while !is_prime(r) {
            r += 1;
        }
        n ^= r + 1;
        ans.push(r);
    }
    for _ in 0..n {
        ans.push(1);
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{}{}", ans[i], if i + 1 == ans.len() { "\n" } else { " " });
    }
}
