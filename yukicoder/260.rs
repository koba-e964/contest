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

const MOD: i64 = 1_000_000_007;

// http://kmjp.hatenablog.jp/entry/2015/08/03/0930
fn solve(a: Vec<i64>) -> i64 {
    let n = a.len();
    let mut dp = vec![vec![vec![vec![vec![0_i64; 8]; 3]; 2]; 2]; n + 1];
    dp[0][0][0][0][0] = 1;
    for i in 0 .. n {
        for more in 0 .. 2 {
            for i3 in 0 .. 2 {
                for m3 in 0 .. 3 {
                    for m8 in 0 .. 8 {
                        for x in 0 .. 10 {
                            if more == 0 && x > a[i] as usize { continue; }
                            let delta = dp[i][more][i3][m3][m8];
                            let ret = &mut dp[i + 1]
                                [more | (if x < a[i] as usize { 1 } else { 0 })]
                                [i3 | (if x == 3 { 1 } else { 0 })]
                                [(m3 * 10 + x) % 3]
                                [(m8 * 10 + x) % 8];
                            *ret += delta;
                            *ret %= MOD;
                        }
                    }
                }
            }
        }
    }
    let mut tot: i64 = 0;
    for more in 0 .. 2 {
        for i3 in 0 .. 2 {
            for m3 in 0 .. 3 {
                for m8 in 0 .. 8 {
                    if (i3 == 1 || m3 == 0) && m8 != 0 {
                        tot = (tot + dp[n][more][i3][m3][m8]) % MOD;
                    }
                }
            }
        }
    }
    tot
}

fn dec(mut a: Vec<i64>) -> Vec<i64> {
    let n = a.len();
    for i in (0 .. n).rev() {
        match a[i] {
            0 => a[i] = 9,
            _ => {
                a[i] -= 1;
                return a;
            }
        }
    }
    panic!();
}

fn main() {
    let a: Vec<i64> = get_word().chars()
        .map(|c| (c as u8 - 48) as i64).collect();
    let b: Vec<i64> = get_word().chars()
        .map(|c| (c as u8 - 48) as i64).collect();
    let ares = solve(dec(a));
    let bres = solve(b);
    println!("{}", (bres - ares + MOD) % MOD);
}
