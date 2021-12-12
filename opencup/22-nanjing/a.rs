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
    let mut a: usize = get();
    let mut b: usize = get();
    let mut x = 'U';
    let mut y = 'L';
    if n + 1 < 2 * a {
        a = n + 1 - a;
        x = 'D';
    }
    if n + 1 < 2 * b {
        b = n + 1 - b;
        y = 'R';
    }
    let mut tmp = "".to_string();
    tmp.push(x);
    tmp.push(y);
    let mut s = tmp.repeat(n - 1);
    for _ in 1..a {
        s.push((x as u8 ^ b'U' ^ b'D') as char);
    }
    for _ in 1..b {
        s.push((y as u8 ^ b'L' ^ b'R') as char);
    }
    println!("{}", s);
}
