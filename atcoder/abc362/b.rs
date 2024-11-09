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

fn dist(x: (i64, i64), y: (i64, i64)) -> i64 {
    let dx = x.0 - y.0;
    let dy = x.1 - y.1;
    dx * dx + dy * dy
}

fn main() {
    let mut p = vec![];
    for _ in 0..3 {
        let x: i64 = get();
        let y: i64 = get();
        p.push((x, y));
    }
    let mut d = [
        dist(p[0], p[1]),
        dist(p[1], p[2]),
        dist(p[2], p[0]),
    ];
    d.sort();
    println!("{}", if d[0] + d[1] == d[2] { "Yes" } else { "No" });
}
