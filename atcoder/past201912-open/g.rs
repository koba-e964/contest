#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};

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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    let n: usize = get();
    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            let x: i64 = get();
            a[i][j] = x;
            a[j][i] = x;
        }
    }
    const INF: i64 = 1 << 50;
    let mut dp = vec![vec![-INF; 4]; 1 << n];
    let mut once = vec![-INF; 1 << n];
    dp[0][0] = 0;
    for bits in 0..1 << n {
        let mut tot = 0;
        for i in 0..n {
            for j in 0..i {
                if (bits & 1 << i) != 0 && (bits & 1 << j) != 0 {
                    tot += a[i][j];
                }
            }
        }
        once[bits] = tot;
    }
    for bits in 1..1 << n {
        for k in 1..4 {
            for sub in 0..bits {
                if (sub & bits) != sub {
                    continue;
                }
                dp[bits][k] = max(dp[bits][k], dp[sub][k - 1] + once[bits - sub]);
            }
        }
    }
    let mut mi = -INF;
    for k in 0..4 {
        mi = max(mi, dp[(1 << n) - 1][k]);
    }
    puts!("{}\n", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
