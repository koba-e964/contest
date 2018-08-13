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

fn mul(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let n = a.len();
    let mut ans = vec![vec![0; n]; n];
    for i in 0 .. n {
        for j in 0 .. n {
            for k in 0 .. n {
                ans[i][j] ^= a[i][k] & b[k][j];
            }
        }
    }
    ans
}

fn pow(a: &[Vec<i32>], mut t: i64) -> Vec<Vec<i32>> {
    let n = a.len();
    let mut sum = vec![vec![0; n]; n];
    for i in 0 .. n { sum[i][i] = 1; }
    let mut cur = a.to_vec();
    while t > 0 {
        if t % 2 == 1 {
            sum = mul(&sum, &cur);
        }
        cur = mul(&cur, &cur);
        t /= 2;
    }
    sum
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n = get();
    let mut a = vec![vec![0; n]; n];
    for i in 0 .. n {
        for j in 0 .. n {
            a[i][j] = get();
        }
    }
    let mut v: Vec<i32> = (0 .. n).map(|_| get()).collect();
    let t: i64 = get();
    let mut a = pow(&a, t);
    // Gaussian elimination
    let mut col = 0;
    let mut row = 0;
    while row < n && col < n {
        let mut cur = n;
        for j in row .. n {
            if a[j][col] == 1 {
                cur = j;
                break;
            }
        }
        if cur == n {
            col += 1;
            continue;
        }
        a.swap(row, cur);
        v.swap(row, cur);
        for i in 0 .. n {
            if i != row {
                if a[i][col] == 1 {
                    for j in 0 .. n {
                        a[i][j] ^= a[row][j];
                    }
                    v[i] ^= v[row];
                }
            }
        }
        row += 1;
        col += 1;
    }
    if row == n {
        for i in 0 .. n {
            puts!("{}{}", v[i], if i == n - 1 { "\n" } else { " " });
        }
        return;
    }
    // check if the given constraints are contradictory
    let mut contra = false;
    for i in row .. n {
        if v[i] == 1 {
            contra = true;
            break;
        }
    }
    puts!("{}\n", if contra { "none" } else { "ambiguous" });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
