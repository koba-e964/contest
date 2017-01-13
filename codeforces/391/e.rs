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

#[allow(dead_code)]
fn factorize (mut n: u64, result: &mut HashMap<u64, usize>) {
    let mut p: u64 = 2;
    while n > 1 && p * p <= n {
        let mut cnt = 0;
        while n % p == 0 {
            cnt += 1;
            n /= p;
        }
        if cnt > 0 {
            let nexp = match result.get(&p) {
                None => cnt,
                Some(orig) => orig + cnt,
            };
            result.insert(p, nexp);
        }
        p += 1;
    }
    if n > 1 {
        let nexp = match result.get(&p) {
            None => 1,
            Some(orig) => orig + 1,
        };
        result.insert(p, nexp);
    }
}

const MOD: i64 = 1_000_000_007;

fn main() {
    let q: usize = parse(&getline().trim());
    const W: usize = 1_000_010;
    const B: usize = 21;
    let mut tbl = vec![vec![0_i64; B]; W];
    tbl[0][0] = 1;
    for i in 1 .. W {
        tbl[i][0] = 1;
        for j in 1 .. B {
            tbl[i][j] = (tbl[i][j - 1] + tbl[i - 1][j]) % MOD;
        }
    }
    for i in 0 .. W {
        let cmb = tbl[i].clone();
        for j in 1 .. B {
            tbl[i][j] = (tbl[i][j - 1] + cmb[j] + cmb[j - 1]) % MOD;
        }
    }
    let mut minfc = vec![0_usize; W];
    for i in 2 .. 2000 {
        for j in 2 .. ((W - 1) / i + 1) {
            if minfc[i * j] == 0 {
                minfc[i * j] = i;
            }
        }
    }
    for i in 2 .. W {
        if minfc[i] == 0 {
            minfc[i] = i;
        }
    }
    for _ in 0 .. q {
        let rn: Vec<_> = getline().trim().split(" ").map(|v| parse(v)).collect();
        let r: usize = rn[0];
        let mut n = rn[1];
        // factorize n
        let mut cur = 1;
        let mut cnt = 0;
        let mut sum = 1;
        while cur > 0 {
            let tmp = minfc[n];
            if cur != tmp {
                sum *= tbl[r][cnt];
                sum %= MOD;
                cur = tmp;
                cnt = 1;
            } else {
                cnt += 1;
            }
            if tmp > 0 {
                n /= tmp;
            }
        }
        println!("{}", sum);
    }
}
