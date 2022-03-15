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

fn calc2(x: i64, y: i64, a: i64, b: i64) -> bool {
    if a + b > x * y {
        return false;
    }
    (a + x - 1) / x + (b + x - 1) / x <= y
        || (a + y - 1) / y + (b + y - 1) / y <= x
}

fn calc(x: i64, y: i64, a: i64, b: i64, c: i64) -> bool {
    if a + b + c > x * y {
        return false;
    }
    if calc2(x, y - (a + x - 1) / x, b, c) {
        return true;
    }
    if calc2(x, y - (b + x - 1) / x, a, c) {
        return true;
    }
    calc2(x, y - (c + x - 1) / x, b, a)
}

fn main() {
    let x: i64 = get();
    let y: i64 = get();
    let a: i64 = get();
    let b: i64 = get();
    let c: i64 = get();
    println!("{}", if calc(x, y, a, b, c) || calc(y, x, a, b, c) {
        "Yes"
    } else {
        "No"
    });
}
