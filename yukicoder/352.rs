use std::cmp::*;
use std::io::*;
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn getword() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            stdin.read(&mut u8b);
            if u8b[0] <= ' ' as u8 {
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
fn parse<T : std::str::FromStr>(s : &str) -> T {
    match s.parse::<T>() {
        Ok(t) => t,
        _    => panic!(),
    }
}

fn calc(n: i32) -> f64 {
    // 1.0 * 2 / n + (n - 2) / n * (sum_{i < j = 1}^n 1 / (n * (n - 1) / 2) * i * j)[n := n - 1]
    // = 2 / n + (n - 2) / n * (1 / n(n-1) * ((n^4 + 2n^3 + n^2) / 4 - (2n^3 + 3n^2 + n) / 6))[n := n - 1]
    // = 2 / n + (n - 2) / n * (1 / n(n-1) * ((3n^4 + 2n^3 - 3n^2 - 2n) / 12))[n := n - 1]
    // = 2 / n + (n - 2) * (3n - 1) / 12
    let nf = n as f64;
    return 2.0 / nf + (nf - 2.0) * (3.0 * nf - 1.0) / 12.0;
}

fn main() {
    let n: i32 = parse(&getword());
    let mut ret: f64 = 1.0;
    for i in 2 .. (n + 1) {
        ret += calc(i);
    }
    println!("{}", ret);
}
