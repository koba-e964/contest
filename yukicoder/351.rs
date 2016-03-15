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

fn main() {
    let h: i32 = parse(&getword());
    let w: i32 = parse(&getword());
    let n: usize = parse(&getword());
    let mut s = vec![' '; n];
    let mut k = vec![0; n];
    let mut x = 0;
    let mut y = 0;
    for i in 0 .. n {
        s[i] = getword().chars().next().unwrap();
        k[i] = parse(&getword());
    }
    for i in (0 .. n).rev() {
        match s[i] {
            'R' => if y == k[i] { x = (x + w - 1) % w },
            'C' => if x == k[i] { y = (y + h - 1) % h },
            _ => panic!(),
        };
    }
    println!("{}", if (x + y) % 2 == 0 { "white" } else { "black" });
}
