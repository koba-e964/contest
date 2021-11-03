use std::collections::*;
use std::io::Read;

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
    let n: usize = get();
    let m: i64 = get();
    let k: i64 = get();
    let nn = n as i64;
    let rem = m - (nn - 2) * (nn - 1) / 2;
    let mut v = k;
    let mut que: VecDeque<_> = (0..nn - 1).collect();
    que.push_back(rem);
    let mut ans = vec![];
    for i in 0..n {
        let len = (n - i - 1) as i64;
        if v >= len {
            ans.push(que.pop_back().unwrap());
            v -= len;
        } else {
            ans.push(que.pop_front().unwrap());
        }
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
