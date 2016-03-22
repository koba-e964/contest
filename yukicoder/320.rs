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
    let m: i64 = parse(&getword());
    let mut fib = vec![1_i64; n + 1];
    fib[0] = 0;
    for i in 3 .. n + 1 {
        fib[i] = fib[i - 1] + fib[i - 2]; 
    }
    if m > fib[n] {
        println!("-1");
        return;
    }
    // greedily chooses fib's
    let mut diff: i64 = fib[n] - m;
    let mut cnt = 0;
    
    for i in (1 .. n - 1).rev() {
        if diff > 0 && diff >= fib[i] {
            diff -= fib[i];
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
