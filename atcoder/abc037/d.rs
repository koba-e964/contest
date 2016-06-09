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

const MOD: i64 = 1_000_000_007;

fn rec(dp: &mut Vec<Vec<i64>>, a: &Vec<Vec<i32>>, x: i32, y: i32) -> i64 {
    let h = a.len();
    let w = a[0].len();
    if dp[x as usize][y as usize] >= 0 {
        return dp[x as usize][y as usize];
    }
    let mut sum: i64 = 1;
    let dxy = [0, -1, 0, 1, 0];
    for d in 0 .. 4 {
        let nx = x + dxy[d];
        let ny = y + dxy[d + 1];
        if nx < 0 || ny < 0 || (nx as usize) >= h || (ny as usize) >= w {
            continue;
        }
        if a[nx as usize][ny as usize] < a[x as usize][y as usize] {
            sum += rec(dp, a, nx, ny);
            sum %= MOD;
        }
    }

    dp[x as usize][y as usize] = sum;
    return sum;
}
fn main() {
    let h: usize = get();
    let w: usize = get();
    let a: Vec<Vec<i32>> = (0 .. h).
        map(|_| (0 .. w).map(|_| get()).collect()).collect();
    let mut dp: Vec<Vec<i64>> = (0 .. h).map(|_| vec![-1; w]).collect();
    let mut sum: i64 = 0;
    for x in 0 .. h {
        for y in 0 .. w {
            sum += rec(&mut dp, &a, x as i32, y as i32);
            sum %= MOD;
        }
    }

    println!("{}", sum);
}
