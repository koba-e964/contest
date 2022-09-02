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

fn solve(c: char, e: i64) {
    if e == 0 {
        println!("{}", if c == 'a' { "a" } else { "aa" });
        return;
    }
    let mut ans = "{".to_string();
    for i in 0..60 {
        if (e & ((1 << i) - 1)) != 0 {
            break;
        }
        if (e >> i) >= 26 { continue; }
        let mut s = "".to_string();
        let mut pos = 0;
        for now in b'a'..c as u8 {
            if pos < e >> i {
                s.push(now as char);
            }
            pos += 1;
        }
        for _ in 0..i {
            s.push(c);
        }
        for now in c as u8 + 1..=b'z' {
            if pos < e >> i {
                s.push(now as char);
            }
            pos += 1;
        }
        if ans > s { ans = s; }
    }
    println!("{}", if ans == "{" { "-1" } else { &ans });
}

fn main() {
    let q: usize = get();
    for _ in 0..q {
        let c: char = get();
        let e: i64 = get();
        solve(c, e);
    }
}
