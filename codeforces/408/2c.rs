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

fn check(a: &[i64], x: i64, edges: &[Vec<usize>]) -> bool {
    let n = a.len();
    let mut diff = vec![0; n];
    let mut one = Vec::new();
    let mut two = Vec::new();
    for i in 0 .. n {
        if a[i] > x { return false; }
        diff[i] = max(a[i] + 2 - x, 0);
        if diff[i] == 2 { two.push(i); }
        if diff[i] == 1 { one.push(i); }
    }
    if two.len() >= 2 { return false; }
    if two.len() == 1 {
        let v = two[0];
        let s = edges[v].iter().collect::<HashSet<_>>();
        for w in one {
            if !s.contains(&w) { return false; }
        }
        return true;
    }
    match one.len() {
        0 => true,
        1 => true,
        _ => {
            let mut s: HashSet<_> = edges[one[0]].iter().cloned().collect();
            s.insert(one[0]);
            let mut t = HashSet::new();
            for &w in edges[one[1]].iter() {
                if s.contains(&w) {
                    t.insert(w);
                }
            }
            if s.contains(&one[1]) {
                t.insert(one[1]);
            }
            assert!(t.len() <= 2);
            for v in t {
                let mut ok = true;
                let mut s = edges[v].iter().cloned().collect::<HashSet<_>>();
                s.insert(v);
                for &w in one.iter() {
                    if !s.contains(&w) { ok = false; break; }
                }
                if ok {
                    return true;
                }
            }
            return false;
        },
    }
}

fn solve() {
    let n = get();
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut edges = vec![Vec::new(); n];
    for _ in 0 .. n - 1 {
        let a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        edges[a].push(b);
        edges[b].push(a);
    }
    let mut lo = -1 << 40;
    let mut hi = 1 << 40;
    while hi - lo > 1 {
        let mid = (hi + lo) / 2;
        // Check if mid is appropriate
        if check(&a, mid, &edges) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    println!("{}", hi);
        
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
