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

fn calc(x: i64, y: i64) -> Vec<(i64, i64)> {
    let mut v = vec![];
    for dx in -2i64..3 {
        for dy in -2i64..3 {
            if dx.abs() + dy.abs() == 3 {
                v.push((x + dx, y + dy));
            }
        }
    }
    v
}

fn main() {
    let x1: i64 = get();
    let y1: i64 = get();
    let x2: i64 = get();
    let y2: i64 = get();
    let p1 = calc(x1, y1);
    let p2 = calc(x2, y2);
    for &v1 in &p1 {
        for &v2 in &p2 {
            if v1 == v2 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
