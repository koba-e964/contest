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
    let n = 80 - get::<usize>();
    let p0: f64 = get::<f64>() / 100.0;
    let p1: f64 = get::<f64>() / 100.0;
    let p2: f64 = get::<f64>() / 100.0;
    const B: usize = 14;
    let mut decay = vec![vec![0.0; B]; 1 << B];
    for bits in 0 .. 1 << B {
        for j in 0 .. B {
            if (bits & 1 << j) == 0 { continue; }
            let p;
            let mut adj = 0;
            if j == 0 || (j >= 1 && (bits & 1 << (j - 1)) == 0) {
                adj += 1;
            }
            if j == B - 1 || (j < B - 1 && (bits & 1 << (j + 1)) == 0) {
                adj += 1;
            }
            p = match adj {
                0 => p2,
                1 => p1,
                2 => p0,
                _ => panic!(),
            } + 1e-8;
            decay[bits][j] = p;
        }
    }
    let mut dp = vec![vec![0.0; 1 << B]; n + 1];
    dp[0][(1 << B) - 1] = 1.0;
    for i in 0 .. n {
        for b1 in 0 .. 1 << B {
            let mut b2 = b1;
            let mut prob = 1.0;
            for j in 0 .. B {
                if (b1 & 1 << j) == 0 { continue; }
                prob *= 1.0 - decay[b1][j];
            }
            loop {
                /*
                for j in 0 .. B {
                    if (b1 & 1 << j) == 0 { continue; }
                    let p = decay[b1][j];
                    prob *= if (b2 & 1 << j) == 0 { p } else { 1.0 - p };
                }
                 */
                dp[i + 1][b2] += prob * dp[i][b1];
                if b2 == 0 {
                    break;
                }
                let newb2 = (b2 - 1) & b1;
                for j in 0 .. B {
                    if (b1 & 1 << j) == 0 { continue; }
                    if (b2 & 1 << j) == (newb2 & 1 << j) { break; }
                    if (newb2 & 1 << j) == 0 {
                        prob *= decay[b1][j] / (1.0 - decay[b1][j]);
                    } else {
                        prob *= (1.0 - decay[b1][j]) / decay[b1][j];
                    }
                }
                b2 = newb2;
            }
        }
    }
    let mut tot = 0.0;
    for i in 0 .. 1 << B {
        tot += ((i as u32).count_ones() as f64) * dp[n][i];
    }
    println!("{}", 2.0 * tot);
}
