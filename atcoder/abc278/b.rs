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

fn conf(h: i32, m: i32) -> bool {
    let h1 = h / 10 * 10 + m / 10;
    let m1 = h % 10 * 10 + m % 10;
    h1 <= 23 && m1 <= 59
}

fn main() {
    let h: i32 = get();
    let m: i32 = get();
    let mut d = h * 60 + m;
    while d < 1440 && !conf(d / 60, d % 60) {
        d += 1;
    }
    if d == 1440 {
        println!("0 0");
    } else {
        println!("{} {}", d / 60, d % 60);
    }
}
