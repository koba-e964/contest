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
    let h: usize = get();
    let w: usize = get();
    for _ in 0..h {
        let s = get_word();
        let mut t = String::new();
        let mut a = 0;
        for c in s.chars() {
            if c == 'T' {
                a += 1;
                if a >= 2 {
                    t += "PC";
                    a = 0;
                }
            } else {
                for _ in 0..a {
                    t.push('T');
                }
                t.push(c);
                a = 0;
            }
        }
        for _ in 0..a {
            t.push('T');
        }
        println!("{}", t);
    }
}
