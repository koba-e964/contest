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

const INF: i64 = 1 << 60;

fn calc(a: &[i64], h: i64, w: i64) -> i64 {
    // at most 34 = 2 * 17
    let n = a.len();
    let mut cur = HashSet::new();
    cur.insert((1, 1));
    for i in 0 .. n + 1 {
        let mut nxt = HashSet::new();
        for (x, y) in cur {
            if x >= h && y >= w {
                return i as i64;
            }
            if i < n {
                if x < h {
                    nxt.insert((x * a[i], y));
                }
                if y < w {
                    nxt.insert((x, a[i] * y));
                }
            }
        }
        cur = nxt;
    }
    return INF;
}

fn solve() {
    let a: i64 = get();
    let b: i64 = get();
    let h: i64 = get();
    let w: i64 = get();
    let n = get();
    let mut ary: Vec<i64> = (0 .. n).map(|_| get()).collect();
    ary.sort();
    ary.reverse();
    let mut mi = calc(&ary, (a + h - 1) / h, (b + w - 1) / w);
    mi = min(mi, calc(&ary, (b + h - 1) / h, (a + w - 1) / w));
    println!("{}", if mi == INF { -1 } else { mi });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
