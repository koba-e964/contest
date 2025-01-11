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
    let d: i32 = get();
    let mut tl = vec![];
    for _ in 0..n {
        let t: i32 = get();
        let l: i32 = get();
        tl.push((t, l));
    }
    for i in 1..=d {
        let mut ma = 0;
        for &(t, l) in &tl {
            ma = ma.max(t * (l + i));
        }
        println!("{ma}");
    }
}
