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
    let m: usize = get();
    let mut c = vec![];
    for _ in 0..n {
        c.push(get_word());
    }
    let mut d = vec![];
    for _ in 0..m {
        d.push(get_word());
    }
    let mut p = vec![];
    for _ in 0..m + 1 {
        p.push(get::<i32>());
    }
    let mut ans = 0;
    for c in c {
        let mut idx = m;
        for i in 0..m {
            if c == d[i] {
                idx = i;
                break;
            }
        }
        if idx == m {
            ans += p[0];
        } else {
            ans += p[idx + 1];
        }
    }
    println!("{}", ans);
}
