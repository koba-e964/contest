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
    let n: usize = parse(&getword());
    let q: usize = parse(&getword());
    let mut ary = vec![false; n + 1]; 
    for _ in 0 .. q {
        let l: usize = parse(&getword());
        let r: usize = parse(&getword());
        ary[l - 1] = ! ary[l - 1];
        ary[r] = ! ary[r];
    }
    for i in (0 .. n).rev() {
        ary[i] ^= ary[i + 1];
    }
    for i in 1 .. (n + 1) {
        print!("{}", if ary[i] { "1" } else { "0" });
    }
    println!("");
}
