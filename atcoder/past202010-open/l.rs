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

fn upd(hm: &mut HashMap<i64, i64>, x: i64, val: i64) {
    *hm.entry(x).or_insert(0) += val;
}

fn solve() {
    let n: usize = get();
    let q: usize = get();
    let h: Vec<i64> = (0..n).map(|_| get()).collect();
    let mut diff = vec![0; n - 1];
    for i in 0..n - 1 {
        diff[i] = h[i + 1] - h[i];
    }
    let mut odd = HashMap::new();
    let mut even = HashMap::new();
    for i in 0..n - 1 {
        if i % 2 == 0 {
            upd(&mut even, diff[i], 1);
        } else {
            upd(&mut odd, diff[i], 1);
        }
    }
    let mut odd_delta = 0;
    for _ in 0..q {
        let ty: i32 = get();
        match ty {
            1 => {
                let x: i64 = get();
                odd_delta -= x;
            }
            2 => {
                let x: i64 = get();
                odd_delta += x;
            }
            3 => {
                let u = get::<usize>() - 1;
                let v: i64 = get();
                let (me, other) = if u % 2 == 0 {
                    (&mut even, &mut odd)
                } else {
                    (&mut odd, &mut even)
                };
                if u > 0 {
                    let old = diff[u - 1];
                    diff[u - 1] = old + v;
                    upd(other, old, -1);
                    upd(other, diff[u - 1], 1);
                }
                if u + 1 < n {
                    let old = diff[u];
                    diff[u] = old - v;
                    upd(me, old, -1);
                    upd(me, diff[u], 1);
                }
            }
            _ => panic!(),
        }
        let tot = odd.get(&odd_delta).cloned().unwrap_or(0)
            + even.get(&-odd_delta).cloned().unwrap_or(0);
        println!("{}", tot);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
