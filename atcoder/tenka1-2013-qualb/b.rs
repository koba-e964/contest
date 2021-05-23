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

fn solve() {
    let q: usize = get();
    let l: i64 = get();
    let mut sz = 0;
    let mut st = vec![];
    for _ in 0..q {
        let cmd: String = get_word();
        if cmd == "Push" {
            let n: i64 = get();
            let m: i32 = get();
            st.push((n, m));
            sz += n;
            if sz > l {
                println!("FULL");
                return;
            }
        } else if cmd == "Pop" {
            let mut n: i64 = get();
            if sz < n {
                println!("EMPTY");
                return;
            }
            while let Some((mut f, v)) = st.pop() {
                let x = min(f, n);
                n -= x;
                f -= x;
                sz -= x;
                if f > 0 {
                    st.push((f, v));
                    break;
                }
                if n == 0 {
                    break;
                }
            }
            assert_eq!(n, 0);
        } else if cmd == "Top" {
            if sz == 0 {
                println!("EMPTY");
                return;
            }
            let &(_, v) = st.last().unwrap();
            println!("{}", v);
        } else {
            println!("{}", sz);
        }
    }
    println!("SAFE");
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
