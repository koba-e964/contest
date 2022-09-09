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
    let t: i64 = get();
    let mut all = 0;
    let mut rare = 0;
    for _ in 0..n {
        let name = get_word();
        let r = get_word();
        let rate: i64 = get();
        all += rate;
        if r != "N" && name.contains("Alicia") {
            rare += rate;
        }
    }
    let sum = rare * (t / 10) * 9 + rare * (t % 10);
    println!("{}", sum as f64 / all as f64);
}
