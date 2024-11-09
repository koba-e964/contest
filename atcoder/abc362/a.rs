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
    let mut x = vec![];
    for _ in 0..3 {
        x.push(get::<i32>());
    }
    let c = get_word();
    const INF: i32 = 1 << 30;
    if c == "Red" {
        x[0] = INF;
    }
    if c == "Green" {
        x[1] = INF;
    }
    if c == "Blue" {
        x[2] = INF;
    }
    println!("{}", x.iter().min().unwrap());
}
