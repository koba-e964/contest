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

fn main() {
    let a: i64 = get();
    let mut e = vec![];
    let n = 2 + 40 * 3;
    for i in 0..39 {
        for j in 0..2 {
            for k in 0..2 {
                e.push((1 + j + 2 * i, 1 + k + 2 * i + 2));
            }
        }
    }
    e.push((1, n - 1));
    e.push((2, n - 1));
    e.push((0, 1 + 40 * 2));
    for i in 0..39 {
        e.push((1 + 40 * 2 + i, 1 + 40 * 2 + i + 1));
    }
    for i in 0..40 {
        if (a & 1 << i) != 0 {
            e.push((1 + 40 * 2 + 39 - i, 1 + 2 * i));
        }
    }
    println!("{} {}", n, e.len());
    for (a, b) in e {
        println!("{} {}", a + 1, b + 1);
    }
}
