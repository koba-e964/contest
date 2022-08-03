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
    let n: usize = get();
    let mut red = vec![0; n + 1];
    for i in 1..n + 1 { red[i] = i; }
    let mut v = 2;
    while v * v <= n {
        if red[v * v] != v * v {
            v += 1;
            continue;
        }
        for i in 1..=n / (v * v) {
            while red[i * v * v] % (v * v) == 0 {
                red[i * v * v] /= v * v;
            }
        }
        v += 1;
    }
    let mut freq = vec![0i64; n + 1];
    for i in 1..n + 1 {
        freq[red[i]] += 1;
    }
    let mut ans = 0;
    for i in 1..n + 1 {
        ans += freq[i] * freq[i];
    }
    println!("{}", ans);
}
