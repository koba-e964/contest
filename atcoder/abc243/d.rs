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
    let _n: i64 = get();
    let mut x: i64 = get();
    let s: String = get();
    let mut v = vec![];
    while x > 0 {
        v.push(x % 2);
        x /= 2;
    }
    v.reverse();
    for c in s.chars() {
        if c == 'U' {
            v.pop();
        } else if c == 'L' {
            v.push(0);
        } else {
            v.push(1);
        }
    }
    for d in v {
        x = 2 * x + d;
    }
    println!("{}", x);
}
