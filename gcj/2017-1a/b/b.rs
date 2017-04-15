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

fn cmp_f64(x: f64, y: f64) -> Ordering {
    if x < y {
        return Ordering::Less;
    }
    if x > y {
        return Ordering::Greater;
    }
    Ordering::Equal
}

fn solve() {
    let t: usize = get();
    for case_nr in 1 .. t + 1 {
        let n: usize = get();
        let p: usize = get();
        let r: Vec<i64> = (0 .. n).map(|_| get()).collect();
        let mut q: Vec<Vec<i64>> = vec![Vec::new(); n];
        for i in 0 .. n {
            q[i] = (0 .. p).map(|_| get()).collect();
            q[i].sort();
            q[i].reverse();
        }
        // Use q[i] as a queue
        let mut ans = 0;
        loop {
            let mut que = Vec::new();
            let mut end = false;
            for i in 0 .. n {
                if q[i].len() == 0 {
                    end = true;
                    break;
                }
                que.push((q[i][q[i].len() - 1] as f64 * 10.0 / r[i] as f64, i));
            }
            if end { break; }
            que.sort_by(|&x, &y| cmp_f64(x.0, y.0));
            // Check if que is valid
            let mut lo = -1 << 62;
            let mut hi = 1 << 62;
            for i in 0 .. n {
                let pkg = q[i][q[i].len() - 1];
                let gr = r[i];
                // 9/10 * x <= pkg / gr <= 11/10 * x
                lo = max(lo, (pkg * 10 + gr * 11 - 1) / (gr * 11));
                hi = min(hi, (pkg * 10) / (gr * 9));
            }
            if lo > hi {
                // impossible
                q[que[0].1].pop().unwrap();
            } else {
                for i in 0 .. n {
                    q[i].pop().unwrap();
                }
                ans += 1;
            }
        }
        println!("Case #{}: {}", case_nr, ans);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
