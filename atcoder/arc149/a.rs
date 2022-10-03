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
    let n: usize = get();
    let m: i64 = get();
    let mut c = 1;
    let mut e = 0;
    let mut d = (0, 0);
    while e < n {
        c = c * 10 % (9 * m);
        e += 1;
        for i in 1..10 {
            if (c - 1) * i % (9 * m) == 0 {
                d = (i, e);
            }
        }
    }
    if d.0 == 0 {
        println!("-1");
        return;
    }
    let s = format!("{}", d.0).repeat(d.1);
    println!("{}", s);
}
