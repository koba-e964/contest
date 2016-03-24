#[allow(unused_imports)]
use std::cmp::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
#[allow(dead_code)]
fn getword() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() ||u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
#[allow(dead_code)]
fn parse<T : std::str::FromStr>(s : &str) -> T {
     return s.parse::<T>().ok().unwrap();
}

fn calc2(x: i64, y: i64, d: i64) -> i64 {
    if x < 0 && y < 0 {
        return calc(-x - 1, -y - 1, d) - calc(0, -y - 1, d) - calc(-x - 1, 0, d) + 1;
    }
    if x < 0 {
        return - calc(-x - 1, y, d) + calc(0, y, d);
    }
    if y < 0 {
        return calc(y, x, d);
    }
    assert!(x >= 0 && y >= 0);
    if x + y <= d {
        return (x + 1) * (y + 1);
    }
    if x > y {
        return calc(y, x, d);
    }
    assert!(x <= y);
    if d <= x {
        return (d + 1) * (d + 2) / 2;
    }
    if d <= y {
        return (2 * d - x + 2) * (x + 1) / 2;
    }
    let e = x + y - d;
    return (x + 1) * (y + 1) - e * (e + 1) / 2;
}
fn calc(x: i64, y: i64, d: i64) -> i64 {
    let res = calc2(x, y, d);
    return res;
}

fn main() {
    let (x1, y1): (i64, i64) = (parse(&getword()), parse(&getword()));
    let (x2, y2): (i64, i64) = (parse(&getword()), parse(&getword()));
    let d = parse(&getword());
    println!("{}", calc(x2, y2, d) + calc(x1 - 1, y1 - 1, d) - calc(x2, y1 - 1, d) - calc(x1 - 1, y2, d));
}
