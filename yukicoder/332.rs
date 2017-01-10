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
    let x: i64 = get();
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    const THRESHOLD: i64 = 20000;
    let giants: Vec<(i64, usize)> =
        (0 .. n).map(|i| (a[i], i)).filter(|&(x, _)| x >= THRESHOLD).collect();
    let babies: Vec<(usize, usize)> =
        (0 .. n).map(|i| (a[i], i)).filter(|&(x, _)| x < THRESHOLD)
        .map(|(x, y)| (x as usize, y)).collect();
    let m = giants.len();
    let p = babies.len();
    // process giants
    assert!(m <= 23); // 20000^24 > 10^10000
    let mut tbl = vec![-1_i64; 1 << m];
    for bits in 0 .. (1 << m) {
        let mut tmp = 0;
        for i in 0 .. m {
            if (bits & (1 << i)) != 0 {
                tmp += giants[i].0;
            }
        }
        tbl[bits] = tmp;
    }
    // process babies
    let maxw = p * THRESHOLD as usize + 1;
    let mut dp = vec![vec![false; maxw]; p + 1];
    dp[0][0] = true;
    for i in 0 .. p {
        for j in 0 .. maxw {
            let mut res = dp[i][j];
            if j >= babies[i].0 {
                res |= dp[i][j - babies[i].0];
            }
            dp[i + 1][j] = res;
        }
    }
    let mut giants_ans = None;
    let mut babies_ans = 0;
    for bits in 0 .. (1 << m) {
        let diff = x - tbl[bits];
        if 0 <= diff && diff < maxw as i64 && dp[p][diff as usize] {
            giants_ans = Some(bits);
            babies_ans = diff as usize;
            break;
        }
    }
    match giants_ans {
        None => println!("No"),
        Some(gg) => {
            let mut ans = vec![false; n];
            for i in 0 .. m {
                if (gg & (1 << i)) != 0 {
                    ans[giants[i].1] = true;
                }
            }
            // Subset recovery
            let mut rem = babies_ans;
            for i in (0 .. p).rev() {
                let cur = babies[i].0;
                if rem >= cur && dp[i][rem - cur] {
                    rem -= cur;
                    ans[babies[i].1] = true;
                }
            }
            assert_eq!(rem, 0);
            for i in 0 .. n {
                print!("{}", if ans[i] { "o" } else { "x" });
            }
            println!("");
        }
    }
}
