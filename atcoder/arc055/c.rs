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
/*
 * Z algorithm. Calculates an array a[i] = |lcp(s, s[i...|s|])|,
 * where s is the given string.
 * If n = s.length(), the returned array has length n + 1.
 * E.g. z_algorithm("ababa") = {5, 0, 3, 0, 1, 0}
 * Reference: http://snuke.hatenablog.com/entry/2014/12/03/214243
 * Verified by: AtCoder ARC055-C (http://arc055.contest.atcoder.jp/submissions/1061771)
 */
fn z_algorithm(s: &[char]) -> Vec<usize> {
    let n = s.len();
    let mut ret = vec![0; n + 1];
    ret[0] = n;
    let mut i = 1; let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] { j += 1; }
        ret[i] = j;
        if j == 0 { i += 1; continue; }
        let mut k = 1;
        while i + k < n && k + ret[k] < j {
            ret[i + k] = ret[k];
            k += 1;
        }
        i += k; j -= k;
    }
    ret
}

fn main() {
    let s: Vec<_> = get_word().chars().collect();
    let n = s.len();
    let mut srev = s.clone();
    srev.reverse();
    let z1 = z_algorithm(&s);
    let z2 = z_algorithm(&srev);
    let mut sum: u64 = 0;
    // try a partition s = x y
    for i in (n / 2 + 1) .. (n - 1) {
        assert!(i > n - i);
        assert!(i >= 2);
        // Find maximum a, c s.t. x[0...a] = y[0...a] && x[|x|-c...|x|] = y[|y|-c...|y|]
        let mut a; let mut c;
        a = z1[i];
        c = z2[n - i];
        // #{(i, j) | 1 <= i <= a, 1 <= j <= c, i + j == n - i}
        a = min(a, n - i - 1);
        c = min(c, n - i - 1);
        let tmp = if a == 0 || c == 0 { 0 } else if a + c >= n - i { a + c + i + 1 - n } else { 0 };
        sum += tmp as u64;
    }
    println!("{}", sum);
}
