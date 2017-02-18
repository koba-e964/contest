#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
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
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

fn main() {
    let n = get();
    let s: Vec<char> = get_word().chars().collect();
    let s: Vec<i32> = s.into_iter().map(|x| if x == 'x' { 1 } else { 0 }).collect();
    // Fix the first two
    for b in 0 .. 4 {
        let mut sol = vec![0; n];
        sol[0] = b % 2;
        sol[1] = b / 2;
        for i in 2 .. n {
            sol[i] = (sol[i - 1] + sol[i - 2] + s[i - 1]) % 2;
        }
        // check the consistency with s[0] and s[n - 1]
        if s[0] == (sol[0] + sol[1] + sol[n - 1]) % 2
            && s[n - 1] == (sol[n - 1] + sol[n - 2] + sol[0]) % 2 {
                for c in sol {
                    print!("{}", if c == 1 { "W" } else { "S" });
                }
                println!("");
                return;
            }
    }
    println!("-1");
}
