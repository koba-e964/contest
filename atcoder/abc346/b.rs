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
    let w: i32 = get();
    let b: i32 = get();
    let c = b"wbwbwwbwbwbw";
    for i in 0..12 {
        let mut ww = 0;
        for j in 0..w + b {
            if c[(j + i) as usize % 12] == b'w' {
                ww += 1;
            }
        }
        if ww == w {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
