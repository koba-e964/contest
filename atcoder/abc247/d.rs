use std::cmp::*;
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
    let q: usize = get();
    let mut que = VecDeque::new();
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let x: i64 = get();
            let c: i64 = get();
            que.push_back((c, x));
        } else {
            let mut c: i64 = get();
            let mut ans = 0;
            while c > 0 {
                let (mut cnt, num) = que.pop_front().unwrap();
                let r = min(cnt, c);
                c -= r;
                cnt -= r;
                ans += r * num;
                if cnt > 0 {
                    que.push_front((cnt, num));
                    break;
                }
            }
            println!("{}", ans);
        }
    }
}
