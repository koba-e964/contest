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
    let t: i32 = get();
    for _ in 0..t {
        let mut n: i64 = get();
        let m: i64 = get();
        let k: i64 = get();
        if n >= k {
            n -= k;
            if m - k > 1 {
                n %= m - k;
                n += k;
            } else {
                println!("0");
                continue;
            }
        }
        println!("{}", [6, 2, 4, 8][(n % 4) as usize]);
    }
}
