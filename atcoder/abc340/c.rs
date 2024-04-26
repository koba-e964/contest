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
    let mut n: u64 = get();
    let mut c = 0;
    let mut d = 1;
    let mut tot = 0;
    while n > 1 {
        tot += n * d + c;
        if n % 2 == 1 {
            c = d + c;
        }
        n /= 2;
        d *= 2;
    }
    println!("{}", tot + c * 2);
}
