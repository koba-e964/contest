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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T {
    parse(&getword())
}

fn main() {
    let n: usize = get();
    let m: usize = get();
    let mut items: Vec<(usize, usize, i32)> = Vec::new();
    for _ in 0 .. m {
        let a: usize = get::<usize>();
        let b: usize = get::<usize>();
        let c: i32 = get::<i32>();
        items.push((a, b, c))
    }
    let mut dp = vec![0; 1 << n];
    for bits in 0 .. (1 << n) {
        for i in 0 .. n {
            if (bits & (1 << i)) == 0 {
                continue;
            }
            let mut before = dp[bits ^ (1 << i)];
            for (f, t, s) in items.iter().cloned() {
                if t == i && (bits & (1 << f)) != 0 {
                    before += s;
                }
            }
            dp[bits] = max(dp[bits], before);
        }
    }
    println!("{}", dp[(1 << n) - 1]);
}
