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
    let mut a: u32 = get();
    let b: u32 = get();
    let cc: i64 = get();
    let c = cc.count_ones();
    let ng = |a, b, c| a + b < c || 120 - a - b < c;
    if ng(a, b, c) || ng(a, c, b) || ng(c, b, a) || (a + b + c) % 2 != 0 {
        println!("-1");
        return;
    }
    let mut aa = 0i64;
    let mut k = (a + b - c) / 2;
    for i in 0..60 {
        if (cc >> i) & 1 == 1 {
            if a > k {
                aa |= 1 << i;
                a -= 1;
            }
        } else {
            if k > 0 {
                aa |= 1 << i;
                a -= 1;
                k -= 1;
            }
        }
    }
    assert_eq!(a, 0);
    assert_eq!(k, 0);
    println!("{} {}", aa, cc ^ aa);
}
