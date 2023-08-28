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

// Intersection of
// - 0 <= z <= 1
// - x^2 + y^2 <= (1-z)^2
// - (x-a)^2 + y^2 <= (1-z)^2
// When cut x = b (a/2 <= b <= 1), the area is sqrt(1-b^2) - b^2 ln((1+sqrt(1-b^2))/b)
fn calc(a: f64) -> f64 {
    const W: i64 = 1_000_000;
    let mut ans = 0.0;
    let width = (1.0 - a / 2.0) / W as f64;
    for i in 0..W + 1 {
        let b = 1.0 - i as f64 * width;
        let tmp = if b == 0.0 {
            1.0
        } else {
            (1.0 - b * b).sqrt() - b * b * ((1.0 + (1.0 - b * b).sqrt()) / b).ln()
        };
        if i % 2 == 1 {
            ans += tmp * 4.0;
        } else if i != 0 && i != W {
            ans += tmp * 2.0;
        } else {
            ans += tmp;
        }
    }
    ans * width * 2.0 / 3.0
}

// https://yukicoder.me/problems/no/1319 (3)
// 積分。厳密解を求めるのは面倒なので Simpson 法を使う。
// 平面 x = b で切った部分の面積は厳密に求められるので、それに対して Simpson 法を使う。
// Tags: simpsons-rule
fn main() {
    let r: f64 = get();
    let h: f64 = get();
    let d: f64 = get();
    println!("{}", r * r * h * calc(d / r));
}
