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

// Tags: zeta-2
fn main() {
    let n: usize = get();
    let mut repr = vec![0; n + 1];
    for i in 1..7100 {
        for j in 1..n / i / i + 1 {
            repr[j * i * i] = j;
        }
    }
    let mut f = vec![0i16; n + 1];
    for i in 1..n + 1 {
        f[repr[i]] += 1;
    }
    let mut ans = 0i64;
    for i in 1..n + 1 {
        let x = f[i] as i64;
        ans += x * x;
    }
    println!("{}", ans);
}
