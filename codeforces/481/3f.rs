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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n = get();
    let k = get();
    let mut r: Vec<(i32, usize)> = (0 .. n).map(|i| (get(), i)).collect();
    let mut rorig = vec![0; n];
    for i in 0 .. n { rorig[i] = r[i].0; }
    let mut g = vec![Vec::new(); n];
    for _ in 0 .. k {
        let x = get::<usize>() - 1;
        let y = get::<usize>() - 1;
        g[x].push(y);
        g[y].push(x);
    }
    r.sort();
    let mut stagger = 0;
    let mut prev = -1;
    let mut output = vec![0; n];
    for (i, (u, v)) in r.into_iter().enumerate() {
        if prev != u {
            stagger = 0;
        }
        let mut ans = i - stagger;
        for &w in g[v].iter() {
            if rorig[w] < rorig[v] { ans -= 1; }
        }
        output[v] = ans;
        stagger += 1;
        prev = u;
    }
    for i in 0 .. n {
        puts!("{}{}", output[i], if i == n - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
