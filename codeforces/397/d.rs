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
    let f: Vec<usize> = (0 .. n).map(|_| get::<usize>() - 1).collect();
    let mut freq = vec![0; n];
    let mut inv = vec![2 * n; n];
    // count freq
    for i in 0 .. n {
        freq[f[i]] += 1;
        inv[f[i]] = i;
    }
    let mut imgsize = 0;
    let mut imgmap = Vec::new(); // [m] -> [n]
    let mut invmap = vec![2 * n; n]; // [n] -> [m]
    for i in 0 .. n {
        if freq[i] > 0 {
            imgsize += 1;
            invmap[i] = imgmap.len();
            imgmap.push(i);
        }
    }
    let m = imgsize;
    let h = imgmap;
    let mut g = vec![0; n];
    for i in 0 .. n {
        g[i] = invmap[f[i]];
    }
    // assertion
    let mut ok = true;
    for i in 0 .. n {
        ok &= h[g[i]] == f[i];
    }
    for i in 0 .. m {
        ok &= g[h[i]] == i;
    }
    if !ok {
        println!("-1");
        return;
    }
    println!("{}", m);
    for i in 0 .. n {
        print!("{}{}", g[i] + 1, if i == n - 1 { "\n" } else { " " });
    }
    for i in 0 .. m {
        print!("{}{}", h[i] + 1, if i == m - 1 { "\n" } else { " " });
    }
}
