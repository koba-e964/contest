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
    let mm: i32 = get();
    let dd: i32 = get();
    let mut y: i32 = get();
    let mut m: i32 = get();
    let mut d: i32 = get();
    d += 1;
    if d > dd {
        d = 1;
        m += 1;
    }
    if m > mm {
        m = 1;
        y += 1;
    }
    println!("{y} {m} {d}");
}
