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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn solve() {
    let t: usize = get();
    for case_nr in 0..t {
        let r: usize = get();
        let c: usize = get();
        let k: usize = get::<usize>() - 1;
        let mut s = vec![vec![]; r];
        for i in 0..r {
            s[i] = get_word().bytes().collect();
        }
        let mut dd = vec![vec![0; r + 1]; c];
        for i in 0..r {
            for j in 0..c {
                let val = if s[i][j] == b'X' {
                    1
                } else {
                    0
                };
                dd[j][i + 1] = dd[j][i] + val;
            }
        }
        let mut ans = 0;
        for j in 0..c {
            if s[k][j] == b'X' { ans += 1; }
        }
        // down
        for i in 1..r {
            let mut tmp = i as i64;
            let lo = max(i, k + 1) - i;
            for j in 0..c {
                let pres = dd[j][r] - dd[j][lo] >= r - k
                    || (lo >= 1 && s[lo - 1][j] == b'X');
                if pres { tmp += 1; }
            }
            ans = min(ans, tmp);
        }
        // up
        for i in 1..r {
            let mut tmp = i as i64;
            let hi = min(k + i, r);
            for j in 0..c {
                let pres = dd[j][hi] >= k + 1
                    || (hi < r && s[hi][j] == b'X');
                if pres { tmp += 1; }
            }
            ans = min(ans, tmp);
        }
        println!("Case #{}: {}", case_nr + 1, ans);
    }
}

