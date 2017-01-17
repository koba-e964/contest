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
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
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


/*
 * This solutions was implemented after the author read the editorial.
 *   (http://codeforces.com/blog/entry/49793)
 */
fn solve(n: usize, k: usize) -> Option<()> {
    match k {
        2 => {
            if n <= 4 {
                return None;
            }
            println!("{}", n - 1);
            for i in 1 .. n {
                println!("{} {}", i, i + 1);
            }
        },
        3 => {
            if n <= 3 {
                return None;
            }
            println!("{}", 3 + 2 * (n - 4));
            println!("1 2\n2 3\n3 4");
            for i in 5 .. (n + 1) {
                println!("{} {}", 1, i);
                println!("{} {}", 3, i);
            }
        },
        _ => return None,
    };
    Some(())
}

fn main() {
    let n: usize = get();
    let k: usize = get();
    match solve(n, k) {
        Some(_) => {},
        None => println!("-1"),
    }
}
