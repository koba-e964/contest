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

fn main() {
    let n: usize = get();
    let s: Vec<char> = get_word().chars().collect();
    // Greedy algorithm
    let mut cnt: usize = 0;
    let mut key_map: HashMap<char, usize> = HashMap::new();
    key_map.insert('L', 0);
    key_map.insert('U', 1);
    key_map.insert('D', 2);
    key_map.insert('R', 3);
    let mut a: [usize; 4] = [0; 4];
    for i in 0 .. n {
        let &idx = key_map.get(&s[i]).unwrap();
        if a[3 - idx] > 0 {
            // clear
            for mut v in a.iter_mut() { *v = 0; }
            cnt += 1;
        }
        a[idx] += 1;
    }
    let mut rem: bool = false;
    for j in 0 .. 4 {
        rem |= a[j] > 0;
    }
    println!("{}", cnt + if rem { 1 } else { 0 });
}
