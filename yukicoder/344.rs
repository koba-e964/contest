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

const MD: i32 = 1000;

fn mul(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let x = a.0 * b.0 + 3 * a.1 * b.1;
    let y = a.0 * b.1 + a.1 * b.0;
    return (x % MD, y % MD);
}

fn main() {
    let n: usize = parse(&getword());
    let mut sum = (1, 0);
    let mut cur = (1, 1);
    let mut e = n;
    while e > 0 {
        if e % 2 == 1 {
            sum = mul(sum, cur);
        }
        cur = mul(cur, cur);
        e /= 2;
    }
    println!("{}", (sum.0 * 2 + if n % 2 == 0 { MD - 1 } else { 0 }) % MD);
}
