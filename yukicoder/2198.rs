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
    let m: i64 = get();
    const W: usize = 20000;
    let mut gad = vec![0; W + 1];
    for i in 0..W + 1 {
        gad[i] = (i * (i + 1) / 2) as i64;
    }
    let mut rem = m;
    for i in (1..W + 1).rev() {
        let q = rem / gad[i];
        rem %= gad[i];
        for _ in 0..q {
            print!("c");
        }
        print!("on");
    }
    println!();
}
