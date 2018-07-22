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

struct Iter { bits: Option<usize>, univ: usize }
impl Iterator for Iter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self.bits {
            None => None,
            Some(bits) => {
                let ans = bits;
                self.bits =
                    if bits == 0 { None }
                else { Some((bits - 1) & self.univ) };
                Some(ans)
            }
        }
    }
}
fn subsets(univ: usize) -> Iter {
    Iter {bits: Some(univ), univ }
}

fn solve() {
    loop {
        let n: usize = get();
        if n == 0 { break; }
        let s: Vec<_> = get_word().chars().collect();
        let mut set = vec![0; n + 1];
        let mut self_broken = false;
        for (i, ch) in s.into_iter().enumerate() {
            set[i + 1] = match ch {
                'u' => 0,
                x => {
                    let u = x as u8 - b'0';
                    if (set[i] & 1 << u) != 0 {
                        self_broken = true;
                        0
                    } else {
                        set[i] | 1 << u
                    }
                },
            }
        }
        if self_broken {
            println!("UNSAFE");
            continue;
        }
        let mut nxt_cand = [0; 1 << 10];
        for i in 1 .. n {
            if set[i] != 0 {
                nxt_cand[set[i - 1]] |= set[i] ^ set[i - 1];
            }
        }
        let mut ok = true;
        for bits in 1 .. 1 << 10 {
            let mut dp = vec![0; bits + 1];
            dp[0] = 1;
            for sub1iter in subsets(bits) {
                let sub1 = bits - sub1iter;
                if sub1 == 0 { continue; }
                for sub2 in subsets(sub1) {
                    if sub2 == 0 { break; }
                    if (nxt_cand[sub2] & bits) != 0 {
                        dp[sub1] |= dp[sub1 ^ sub2];
                    }
                }
            }
            if dp[bits] > 0 {
                ok = false;
                break;
            }
        }
        println!("{}", if ok { "SAFE" } else { "UNSAFE" });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
