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

fn ok(n: i64, d: usize) -> bool {
    if d == 0 {
        return n == 0;
    }
    if n >= 1 << (d + 1) {
        return false;
    }
    // subtract [1, 1, 2, 4, ...] from the array
    if n < 1 << (d - 1) {
        return false;
    }
    if ok(n - (1 << (d - 1)), d - 1) {
        return true;
    }
    // subtract [1, 2, 4, 8, ...] from the array
    let val = (1 << d) - 1;
    if n < val {
        return false;
    }
    ok(n - val, d - 1)
}

fn main() {
    let n: i64 = get();
    for d in 0..31 {
        if ok(n, d) {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
