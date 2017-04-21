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

fn rot(v: &[char], x: usize) -> Vec<char> {
    let n = v.len();
    let x = x % n;
    let mut ret = v[x .. n].to_vec();
    let mut u = v[0 .. x].to_vec();
    ret.append(&mut u);
    ret
}

fn solve() {
    let n = get();
    let s: Vec<Vec<char>> = (0 .. n).map(|_| get_word().chars().collect())
        .collect();
    let m = s[0].len();
    // check if rotation makes s[i] equal to s[0].rotate(j)
    let mut dict = HashMap::new();
    for j in 0 .. m {
        let r = rot(&s[0], j);
        if !dict.contains_key(&r) {
            let sz = dict.len();
            dict.insert(r, sz);
        }
    }
    let sz = dict.len();
    const INF: usize = 1 << 23;
    let mut tbl = vec![vec![INF; sz]; n];
    for i in 0 .. n {
        for j in 0 .. m {
            let r = rot(&s[i], j);
            if let Some(&ent) = dict.get(&r) {
                tbl[i][ent] = min(tbl[i][ent], j);
            }
        }
    }
    let mut mi = INF;
    for j in 0 .. sz {
        let mut tot = 0;
        for i in 0 .. n {
            tot += tbl[i][j];
        }
        mi = min(mi, tot);
    }
    if mi != INF {
        println!("{}", mi);
    } else {
        println!("-1");
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
