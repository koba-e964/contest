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
            if res.is_err() || u8b[0] <= ' ' as u8 {
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
    let n: usize = get();
    let mut s: Vec<Vec<i32>> = vec![Vec::new(); n];
    for i in 0 .. n {
        let str = get_word().chars()
            .map(|t|
                 if t == '0' { 0 } else { 1 })
        .collect::<Vec<i32>>();
        assert!(str.len() == n);
        s[i] = str;
    }
    // Gaussian elimination
    let mut r = 0;
    for c in 0 .. n {
        let mut pr = r;
        // look for pr in [r, n) s.t. s[pr][c] = 1
        while pr < n {
            if s[pr][c] == 1 {
                break;
            }
            pr += 1;
        }
        if pr >= n {
            continue;
        }
        // pivot
        if r != pr {
            s.swap(r, pr);
        }
        for qr in (r + 1) .. n {
            if s[qr][c] == 1 {
                // xor
                for j in 0 .. n {
                    s[qr][j] ^= s[r][j];
                }
            }
        }
        r += 1;
    }
    println!("{}", if r == n { "Odd" } else { "Even" });
}
