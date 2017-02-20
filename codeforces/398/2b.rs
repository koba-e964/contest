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
    let ts: i64 = get();
    let tf: i64 = get();
    let t: i64 = get();
    let n = get();
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut endpoint = vec![None; n + 1];
    {
        let mut cur = ts;
        let mut pos = 0;
        while pos < n && cur < tf {
            if a[pos] > cur {
                cur = a[pos];
            }
            endpoint[pos + 1] = Some(cur + t);
            cur += t;
            pos += 1;
        }
    }
    endpoint[0] = Some(ts);
    let mut mi = (1 << 50, -1);
    for i in 1 .. n + 1 {
        if let Some(y) = endpoint[i - 1] {
            if y + t <= tf {
                mi = min(mi, (max(0, y - a[i - 1] + 1), a[i - 1] - 1));
            }
        }
    }
    if let Some(x) = endpoint[n] {
        if x + t <= tf {
            mi = min(mi, (0, x));
        }
    }
    if n <= 0 || a[0] >= tf {
        mi = min(mi, (0, ts));
    }
    println!("{}", mi.1);
}
