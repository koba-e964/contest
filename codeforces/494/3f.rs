#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
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
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

const MOD: i64 = 1_000_000_007;

fn powmod(x: i64, mut e: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % MOD;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % MOD;
        }
        cur = cur * cur % MOD;
        e /= 2;
    }
    sum
}


fn calc_hash(s: &[char], b: &[i64; 2]) -> [i64; 2] {
    let n = s.len();
    let mut ans = [0; 2];
    for j in 0 .. 2 {
        let mut a = 0;
        for i in 0 .. n {
            a = (a * b[j] + s[i] as i64) % MOD;
        }
        ans[j] = a;
    }
    ans
}

fn get_subarray_hash(seq: &[[i64; 2]], start: usize, len: usize) -> [i64; 2] {
    let c = [291, 373];
    let mut hash = [0; 2];
    for j in 0 .. 2 {
        hash[j] = MOD + seq[start + len][j] - seq[start][j] * powmod(c[j], len as i64) % MOD;
        hash[j] %= MOD;
    }
    hash
}

fn solve() {
    let n = get();
    let s: Vec<Vec<char>> = (0 .. n).map(|_| get_word().chars().collect()).collect();
    let b = [147, 152];
    let hashes: Vec<[i64; 2]> = s.iter().map(|s| calc_hash(s, &b)).collect();
    let c = [291, 373];
    let mut seq: Vec<[i64; 2]> = vec![[0; 2]; n + 1];
    for j in 0 .. 2 {
        for i in 0 .. n {
            seq[i + 1][j] = (seq[i][j] * c[j] + hashes[i][j]) % MOD;
        }
    }
    let mut lenacc = vec![0; n + 1];
    for i in 0 .. n {
        lenacc[i + 1] = lenacc[i] + s[i].len() + 1;
    }
    let mut mi = lenacc[n];
    for len in 1 .. n + 1 {
        if n < 2 * len { continue; }
        for i in 0 .. n - 2 * len + 1 {
            let hash = get_subarray_hash(&seq, i, len);
            let mut pos = i + len;
            let mut count = 1;
            while pos + len <= n {
                if get_subarray_hash(&seq, pos, len) == hash {
                    count += 1;
                    pos += len;
                } else {
                    pos += 1;
                }
            }
            if count <= 1 { continue; }
            let total = lenacc[n]
                - count * (lenacc[i + len] - lenacc[i] - 1 - len);
            mi = min(mi, total);
        }
    }
    println!("{}", mi - 1);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
