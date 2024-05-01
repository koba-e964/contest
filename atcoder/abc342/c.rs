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
    let s = get_word().bytes().collect::<Vec<_>>();
    let mut tbl = vec![0; 26];
    for i in 0..26 {
        tbl[i] = i;
    }
    let q: i32 = get();
    for _ in 0..q {
        let c: char = get();
        let d: char = get();
        let c = c as usize - 'a' as usize;
        let d = d as usize - 'a' as usize;
        for i in 0..26 {
            if tbl[i] == c {
                tbl[i] = d;
            }
        }
    }
    for c in s {
        print!("{}", (tbl[(c - b'a') as usize] as u8 + b'a') as char);
    }
}
