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


const DEBUG: bool = false;

#[inline]
fn calc_density(elem: u32, exp: u32) -> f64 {
    if elem > exp {
        return 0.0;
    }
    1.0 / (1u64 << exp + 1 - max(elem, 1)) as f64
}
/* 
 * \sum_{i=0}^elem calc_density(i, exp)
 */
#[inline]
fn calc_density_acc(elem: u32, exp: u32) -> f64 {
    if elem > exp {
        return 1.0;
    }
    1.0 / (1u64 << exp - elem) as f64
}


fn solve(p: &[i64]) -> f64 {
    let n = p.len();
    let exp: Vec<u32> = p.iter().map(|v| (v - 1).trailing_zeros()).collect();
    let mut inex = vec![0.0f64; 1 << n];
    for b2 in (0 .. 1 << n).rev() {
        let mut cur = 1.0;
        for i in 0 .. n {
            if (b2 & (1 << i)) != 0 {
                cur /= p[i] as f64;
            }
        }
        for b3 in b2 + 1 .. 1 << n {
            if (b3 & b2) == b2 {
                cur -= inex[b3];
            }
        }
        inex[b2] = cur;
    }
    let mut dp = vec![1.0_f64; 1 << n];
    let mut norm_prod = vec![vec![1.0; 1 << n]; 64];
    let mut acc_prod = vec![vec![1.0; 1 << n]; 64];
    for sh in 0 .. 64 {
        for bits in 0 .. 1 << n {
            let mut a = 1.0;
            let mut b = 1.0;
            for i in 0 .. n {
                if (bits & 1 << i) != 0 {
                    a *= calc_density(sh, exp[i]);
                    b *= calc_density_acc(sh, exp[i]);
                }
            }
            norm_prod[sh as usize][bits] = a;
            acc_prod[sh as usize][bits] = b;
        }
    }
    for bits in 1 .. 1usize << n {
        let pop = bits.count_ones();
        if pop == 1 {
            dp[bits] = 1.0;
            continue;
        }
        let mut factor = 1.0;
        for i in 0 .. n {
            if (bits & 1 << i) == 0 {
                factor *= p[i] as f64;
            }
        }
        let coprime = inex[(1 << n) - 1 - bits] * factor;
        let mut tot = 1.0;
        let mut loopback = 0.0;
        let mut b2 = (bits - 1) & bits;
        loop {
            if b2 == 0 { break; }
            tot += factor * inex[(1 << n) - 1 - bits + b2]
                * (dp[b2] + dp[bits ^ b2]);
            b2 = (b2 - 1) & bits;
        }
        loopback += factor * inex[(1 << n) - 1];
        b2 = bits;
        let exp_max = exp.iter().max().unwrap();
        loop {
            let mut prob = 0.0;
            for sh in 0 .. exp_max + 0 {
                // Find #c s.t. c^(2^(sh+1)) = 1 and (c^(2^sh) - 1) % prod(b2)p[i] == 0
                if b2 == bits && sh >= 1 {
                    break;
                }
                let sh = sh as usize;
                let cur = acc_prod[sh][b2] * norm_prod[sh + 1][bits - b2];
                prob += cur;
            }
            if DEBUG {
                println!("b2 = {}, prob = {} (tot = {}, loopback = {})", b2, prob, tot, loopback);
            }
            if b2 == 0 || b2 == bits {
                loopback += coprime * prob;
            } else {
                tot += coprime * prob * (dp[b2] + dp[b2 ^ bits]);
            }
            if b2 == 0 {
                break;
            }
            b2 = (b2 - 1) & bits;
        }
        dp[bits] = tot / (1.0 - loopback);
        if DEBUG { 
            println!("[{}] {}, {} ==> {}",  bits, tot, loopback, dp[bits]);
        }
    }
    dp[(1 << n) - 1]
}

fn main() {
    let t = get();
    for _ in 0 .. t {
        let n = get();
        let p: Vec<i64> = (0 .. n).map(|_| get()).collect();
        println!("{}", solve(&p));
    }
}
