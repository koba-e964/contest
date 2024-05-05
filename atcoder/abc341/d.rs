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

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let n: i64 = get();
    let m: i64 = get();
    let k: i64 = get();
    let g = gcd(n, m);
    let l = n / g * m;
    let mut fail = 0;
    let mut pass = n * k * 2;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let cnt = mid / n + mid / m - mid / l * 2;
        if cnt >= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
