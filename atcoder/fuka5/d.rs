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
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
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

/// Generates an Iterator over subsets of univ, in the descending order. 
/// Verified by: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3050308
struct SubsetIter { bits: Option<usize>, univ: usize }
impl Iterator for SubsetIter {
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
fn subsets(univ: usize) -> SubsetIter {
    SubsetIter { bits: Some(univ), univ: univ }
}

fn main() {
    loop {
        let n: usize = get();
        let a: i64 = get();
        let b: i64 = get();
        if n == 0 {
            break;
        }
        let x: Vec<i64> = (0..n).map(|_| get()).collect();
        let mut sum = vec![0; 1 << n];
        for i in 0..n {
            for bits in 0..1 << i {
                sum[(1 << i) + bits] = sum[bits] + x[i];
            }
        }
        let mut dp = vec![0i64; 1 << n];
        for bits in 0usize..1 << n {
            if bits.count_ones() <= 1 {
                dp[bits] = 1;
                continue;
            }
            for i in 0..n {
                if (bits & 1 << i) == 0 {
                    continue;
                }
                for l in subsets(bits ^ 1 << i) {
                    let r = bits ^ 1 << i ^ l;
                    let diff = (sum[l] - sum[r]).abs();
                    if a <= diff && diff <= b {
                        dp[bits] += dp[l] * dp[r];
                    }
                }
            }
        }
        println!("{}", dp[(1 << n) - 1]);
    }
}
