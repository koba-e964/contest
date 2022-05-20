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

// digit sum
fn calc2(mut x: i64) -> i64 {
    let mut s = 0;
    while x > 0 {
        let r = (x % 10) as usize;
        s += [1, 0, 0, 0, 1, 0, 1, 0, 2, 1][r];
        x /= 10;
    }
    s
}

// accumulated
fn calc(x: i64) -> i64 {
    let tbl = [1, 1, 1, 1, 2, 2, 3, 3, 5, 6];
    if x <= 0 { return 0; }
    if x <= 9 { return tbl[(x % 10) as usize] - 1; }
    let q = x / 10;
    let mut s = calc2(q).saturating_mul(x % 10 + 1).saturating_add(tbl[(x % 10) as usize]);
    s = s.saturating_add(calc(q - 1).saturating_mul(10));
    if q >= 1 {
        s = s.saturating_add(q.saturating_mul(5));
        if q >= 2 {
            s = s.saturating_add(q - 1);
        }
    }
    s
}

fn main() {
    let k: i64 = get();
    let mut pass = 0;
    let mut fail = 2 * (k + 9) + 1;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        if k >= calc(mid) {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    if calc(pass) != k {
        println!("-1");
    } else {
        println!("{}", pass);
    }
}
