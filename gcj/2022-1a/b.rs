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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        calc(n);
    }
}

fn calc(n: usize) {
    let mut v = vec![1i64; n];
    for i in 0..30 {
        v[i] = 1 << i;
    }
    for i in 30..n {
        v[i] = (1 << 29) + 1 + i as i64 - 30;
    }
    for i in 0..n {
        print!("{}{}", v[i], if i + 1 == n { "\n" } else { " " });
    }
    let mut res = vec![0i64; n];
    for i in 0..n { res[i] = get(); }
    let mut sum = 0;
    for i in 0..n {
        sum += v[i] + res[i];
    }
    let mut rem = sum / 2;
    let mut sub = vec![];
    for i in 0..n {
        if rem >= res[i] {
            sub.push(res[i]);
            rem -= res[i];
        }
    }
    for i in (0..n).rev() {
        if rem >= v[i] {
            sub.push(v[i]);
            rem -= v[i];
        }
    }
    for i in 0..sub.len() {
        print!("{}{}", sub[i], if i + 1 == sub.len() { "\n" } else { " " });
    }
}
