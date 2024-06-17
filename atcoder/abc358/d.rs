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
    let mut a: Vec<i64> = (0..n).map(|_| get()).collect();
    let mut b: Vec<i64> = (0..m).map(|_| get()).collect();
    a.sort();
    b.sort();
    let mut pos = 0;
    let mut ans = 0;
    for a in a {
        if pos < m && a >= b[pos] {
            pos += 1;
            ans += a;
        }
    }
    println!("{}", if pos < m { -1 } else { ans });
}
