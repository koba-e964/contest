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
    let n = parse(&getword());
    let l: f64 = parse(&getword());
    let mut x = vec![0.0_f64; n];
    for i in 1 .. n {
        x[i] = parse(&getword());
    }
    // Is this structure realisable?
    for i in 0 .. n - 1 {
        if (x[i] - x[i + 1]).abs() >= l {
            println!("-1");
            return;
        }
    }
    let mut acc = vec![0.0; n + 1];
    
    for i in (0 .. n).rev() {
        acc[i] = acc[i + 1] + x[i];
    }
    let mut cnt = 0;
    for i in 0 .. n - 1 {
        let centre = acc[i + 1] / ((n - i - 1) as f64);
        if (x[i] - centre).abs() < l / 2.0 &&
            (x[i + 1] - centre).abs() < l / 2.0 {
            } else {
                cnt += 1;
            }
    }
    println!("{}", cnt);
}
