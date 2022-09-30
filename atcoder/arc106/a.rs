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
    let n: i64 = get();
    let mut c3 = 3;
    let mut e3 = 1;
    while c3 < n {
        let rem = n - c3;
        let mut c5 = 5;
        let mut e5 = 1;
        while c5 < rem {
            c5 *= 5;
            e5 += 1;
        }
        if c5 == rem {
            println!("{} {}", e3, e5);
            return;
        }
        c3 *= 3;
        e3 += 1;
    }
    println!("-1");
}
