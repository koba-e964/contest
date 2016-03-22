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
    let mut c:[f64;4] = [0.0; 4];
    for i in 0 .. 4 {
        c[i] = parse(&getword());
    }
    let s = c[0] * c[2] - c[1] * c[1]; // p_1p_2(z_1 - z_2)^2
    let t = c[1] * c[3] - c[2] * c[2]; // p_1p_2(z_1 - z_2)^2z_1z_2
    let u = c[0] * c[3] - c[1] * c[2]; // p_1p_2(z_1 - z_2)^2(z_1 + z_2)
    let a: f64 = u / s;
    let b: f64 = t / s;
    println!("{}", if a.powi(2) >= 4.0 * b { "R" } else { "I" });
}
