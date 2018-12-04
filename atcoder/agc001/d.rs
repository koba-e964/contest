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
    let m: usize = get();
    let a: Vec<usize> = (0 .. m).map(|_| get()).collect();
    let mut e = 0;
    for i in 0 .. m {
        e += a[i] / 2;
    }
    if e < n - 1 - n / 2 {
        println!("Impossible");
        return;
    }
    let mut odd = Vec::new();
    let mut rest = Vec::new();
    for a in a {
        if a % 2 == 1 {
            odd.push(a);
        } else {
            rest.push(a);
        }
    }
    let mut ans1 = vec![0; m];
    let mut ans2;
    if n % 2 == 1 {
        // a[i] must have exactly one odd element
        assert_eq!(odd.len(), 1);
        let odd = odd[0];
        ans2 = vec![0; m + 1];
        ans1[0] = odd;
        ans2[0] = odd - 1;
        for i in 0 .. m - 1 {
            ans1[i + 1] = rest[i];
            ans2[i + 1] = rest[i];
        }
        ans2[m] = 1;
    } else {
        assert!(odd.len() == 0 || odd.len() == 2);
        if odd.len() == 0 {
            ans1 = rest;
            ans2 = vec![0; m + 1];
            ans2[0] = 1;
            for i in 0 .. m {
                ans2[i + 1] = ans1[i];
            }
            ans2[m] -= 1;
        } else {
            ans2 = vec![0; m];
            ans1[0] = odd[0];
            ans2[0] = odd[0] + 1;
            for i in 0 .. m - 2 {
                ans1[i + 1] = rest[i];
                ans2[i + 1] = rest[i];
            }
            ans2[m - 1] = odd[1] - 1;
            ans1[m - 1] = odd[1];
        }
    }
    ans2 = ans2.into_iter().filter(|&x| x != 0).collect();
    for i in 0 .. ans1.len() {
        print!("{}{}", ans1[i], if i + 1 == ans1.len() { "\n" } else { " " });
    }
    println!("{}", ans2.len());
    for i in 0 .. ans2.len() {
        print!("{}{}", ans2[i], if i + 1 == ans2.len() { "\n" } else { " " });
    }
}
