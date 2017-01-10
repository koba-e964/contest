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
 * Manages multiple linear graphs.
 * Lines that are not necessary to calculate maximum values are deleted.
 * Verified by: yukicoder No.409 (http://yukicoder.me/submissions/143613)
 */
struct ConvexHullTrick {
    dat: Vec<(i64, i64)>, // (a,b) -> y = a * x + b
    cur_idx: usize, // current index (in 0 .. dat.len())
}

impl ConvexHullTrick {
    fn new() -> Self {
        ConvexHullTrick { dat: Vec::new(), cur_idx: 0, }
    }
    fn check(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
        (b.0 - a.0) * (c.1 - b.1) >= (b.1 - a.1) * (c.0 - b.0)
    }
    /*
     * added.0 must be the largest.
     */
    fn add(&mut self, added: (i64, i64)) {
        while self.dat.len() >= 2 {
            let l = self.dat.len();
            if Self::check(self.dat[l - 2], self.dat[l - 1], added) {
                if self.cur_idx == self.dat.len() - 1 {
                    self.cur_idx -= 1;
                }
                self.dat.pop().unwrap();
            } else {
                break;
            }
        }
        self.dat.push(added);
    }
    #[allow(dead_code)]
    fn get(&self) -> Vec<(i64, i64)> {
        self.dat.clone()
    }
    // The caller must ensure that x is increasing,
    // when calls are sorted in chronological order.
    fn query(&mut self, x: i64) -> i64 {
        let n = self.dat.len();
        while self.cur_idx < n - 1 {
            let line = self.dat[self.cur_idx];
            let line2 = self.dat[self.cur_idx + 1];
            if line.0 * x + line.1 < line2.0 * x + line2.1 {
                self.cur_idx += 1;
            } else {
                break;
            }
        }
        let line = self.dat[self.cur_idx];
        line.0 * x + line.1
    }
}

const MINF: i64 = -1_i64 << 60;

fn main() {
    let n: usize = get();
    let a: i64 = get();
    let b: i64 = get();
    let w: i64 = get();
    let d: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut dp: Vec<i64> = vec![MINF; n + 2];
    // dp[0] = 0
    // dp[i] = d[i - 1] + min (dp[t] - a(i-t-1) + b * (i - t) * (i - t - 1) / 2)
    // 2dp[i] = d[i-1] - 2a(i-1) + bi^2 + min(2dp[t] + bt(t+1) + 2at - b(2t+1)i)
    let mut cht = ConvexHullTrick::new();
    dp[0] = 0;
    for i in 1 .. (n + 2) {
        let t = i as i64 - 1;
        // Signs are flipped because cht calculates the maximum value.
        cht.add((b * (2 * t + 1),
                 -2 * dp[i - 1] - b * t * (t + 1) - 2 * a * t));
        let ii = i as i64;
        let tmp =
            -cht.query(ii) + 2 * (if i == n + 1 { 0 } else { d[i - 1] })
            - 2 * a * (ii - 1) + b * ii * ii;
        assert!(tmp % 2 == 0);
        dp[i] = tmp / 2;
    }
    println!("{}", w + dp[n + 1]);
}
