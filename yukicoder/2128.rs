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

fn gcd(mut x: i128, mut y: i128) -> i128 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn calc(x: i128, a: i128, b: i128) -> i128 {
    assert!(a <= b);
    let g = gcd(a, b);
    let l = a / g * b;
    let ma = (x + a - 1) / a * a;
    let mb = (x + b - 1) / b * b;
    let ml = (x + l - 1) / l * l;
    if ma >= mb {
        return (ml - mb) / b * 2 + 1 + if mb == x { 0 } else { 1 };
    }
    (ml - mb) / b * 2 + 2 + if ma == x { 0 } else { 1 }
}

fn main() {
    let t: usize = get();
    for _ in 0..t {
        let x: i128 = get();
        let a: i128 = get();
        let b: i128 = get();
        println!("{}", if a < b { calc(x, a, b) } else { calc(x, b, a) } % 998_244_353);
    }
}
