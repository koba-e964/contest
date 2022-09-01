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
    let a: f64 = get();
    let b: f64 = get();
    let c: f64 = get();
    let mut pass = 1.0;
    let mut fail = 2.0;
    for _ in 0..100 {
        let mid = (pass + fail) / 2.0;
        let val = (a * mid * mid * mid * mid + b) * mid + c;
        if val < 0.0 {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
