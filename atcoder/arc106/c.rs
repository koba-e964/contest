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
    let m: i32 = get();
    if m < 0 || (n >= 2 && m >= n as i32 - 1) {
        println!("-1");
        return;
    }
    let m = m as usize;
    if m == 0 {
        for i in 0..n {
            println!("{} {}", 2 * i + 1, 2 * i + 2);
        }
        return;
    }
    for i in 0..n - 2 - m {
        println!("{} {}", 2 * i + 1, 2 * i + 2);
    }
    println!("{} {}", 2 * (n - 2 - m) + 1, 1_000_000_000);
    for i in n - m - 1..n {
        println!("{} {}", 2 * i + 1, 2 * i + 2);
    }
}
