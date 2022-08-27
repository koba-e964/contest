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
    let mut ans = 3.5;
    let tbl = [0.0, 1.0, 3.0, 6.0, 10.0, 15.0, 21.0];
    for _ in 0..n - 1 {
        let mut ma = 0.0f64;
        for i in 0..7 {
            let tmp = ans * i as f64 / 6.0 + (21.0 - tbl[i]) / 6.0;
            ma = ma.max(tmp);
        }
        ans = ma;
    }
    println!("{}", ans);
}
