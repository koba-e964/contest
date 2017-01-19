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

fn numero(v: &[i64], b: i64) -> Option<i64> {
    let mut cur: i64 = 0;
    for &t in v.iter() {
        cur = match cur.checked_mul(b) {
            Some(v) => v,
            None => return None,
        };
        cur = match cur.checked_add(t) {
            Some(v) => v,
            None => return None,
        }
    }
    Some(cur)
}

fn checked_propagate(b: i64, x: i64, y: i64) -> Option<i64> {
    let cur = match x.checked_mul(b) {
        Some(v) => v,
        None => return None,
    };
    cur.checked_add(y)
}


fn main() {
    let b: i64 = get();
    let s: Vec<_> = get_word().bytes().map(|b| b as i64 - 0x30).collect();
    let n = s.len();
    if b <= 9 {
        // simply read b-base rep.
        let mut cur = 0;
        for v in s {
            cur *= b;
            cur += v;
        }
        println!("{}", cur);
        return;
    }
    let mut dp = vec![-1i64; n + 1];
    const INF: i64 = 1i64 << 60;
    dp[0] = 0;
    for i in 1 .. n + 1 {
        let mut mi = INF;
        for j in 1 .. i + 1 {
            // s[i - j ..i] < b ?
            if j >= 10 || dp[i - j] < 0 || (j >= 2 && s[i - j] == 0) {
                continue;
            }
            let sl = numero(&s[i - j .. i], 10);
            match sl {
                Some(v) if v < b => match checked_propagate(b, dp[i - j], v) {
                    Some(v) => { mi = min(mi, v) },
                    None => {},
                },
                _ => {},
            }
        }
        if mi < INF {
            dp[i] = mi;
        }
    }
    //println!("{:?}", dp);
    println!("{}", dp[n]);
}
