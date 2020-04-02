#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let k = 42;
    let n = k * (k - 1) + 1;
    let mut ans = vec![vec![]; n];
    for i in 0..k {
        ans[i].push(0);
        for j in 0..k - 1 {
            ans[i].push(i * (k - 1) + j + 1);
        }
    }
    for i in k..n {
        let a = (i - k) / (k - 1);
        let b = (i - k) % (k - 1);
        ans[i].push(a + 1);
        for j in 1..k {
            ans[i].push((b + (j - 1) * a) % (k - 1) + j * (k - 1) + 1);
        }
    }
    puts!("{} {}\n", n, k);
    for i in 0..n {
        for j in 0..k {
            puts!("{}{}", ans[i][j] + 1, if j + 1 == k { "\n" } else { " " });
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
