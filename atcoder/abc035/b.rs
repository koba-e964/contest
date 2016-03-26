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

fn main() {
    let s = getword();
    let t: usize = parse(&getword());
    let mut x = 0_i32;
    let mut y = 0_i32;
    let mut q = 0_i32;
    for c in s.chars() {
        match c {
            'L' => x -= 1,
            'R' => x += 1,
            'U' => y += 1,
            'D' => y -= 1,
            '?' => q += 1,
            _   => panic!(),
        }
    }
    let d = x.abs() + y.abs();
    println!("{}", if t == 1 {
        d + q
    } else {
        if d >= q {
            d - q
        } else {
            (q - d) % 2
        }
            
    });
}
