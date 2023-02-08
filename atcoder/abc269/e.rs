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

fn ask(ab: std::ops::Range<usize>, cd: std::ops::Range<usize>) -> usize {
    println!("? {} {} {} {}", ab.start + 1, ab.end, cd.start + 1, cd.end);
    let x: i64 = get();
    if x == -1 { panic!() }
    x as usize
}

fn main() {
    let n: usize = get();
    let mut pass = 0;
    let mut fail = n;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        if ask(0..mid, 0..n) == mid {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let x = fail;
    let mut pass = 0;
    let mut fail = n;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        if ask(0..n, 0..mid) == mid {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let y = fail;
    println!("! {} {}", x, y);
}
