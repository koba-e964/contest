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

fn rev(mut a: i64) -> i64 {
    let mut ret = 0;
    while a > 0 {
        ret = ret * 10 + a % 10;
        a /= 10;
    }
    ret
}

fn main() {
    let n: i64 = get();
    let mut ans = 0;
    for i in 1..1_000_001i64 {
        let c = i * i * i;
        if c <= n && c == rev(c) {
            ans = c;
        }
    }
    println!("{}", ans);
}
