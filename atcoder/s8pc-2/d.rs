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

const LIM: i64 = 100_000_000_000_000_000;
// const LIM: i64 = 100_000_000;

const N: usize = 15;
fn precompute() -> HashMap<i64, i64> {
    let factor_base: [i64; N] = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47];
    let mut map = HashMap::new();
    let mut que = VecDeque::new();
    que.push_back(([0usize; N], 1i64, 0));
    while let Some((state, val, start)) = que.pop_front() {
        // count #factors
        let mut num_fac = 1;
        for i in 0 .. N {
            num_fac *= state[i] as i64 + 1;
        }
        let ent = map.entry(num_fac).or_insert(LIM);
        *ent = min(*ent, val);
        for i in start .. N {
            let f = factor_base[i];
            if val * f > LIM {
                break;
            }
            if i >= 1 && state[i] == state[i - 1] {
                continue;
            }
            let mut state_cp = state;
            state_cp[i] += 1;
            que.push_back((state_cp, val * f, i));
        }
    }
    map // it is known that |map| = 2026
}


fn solve() {
    let q = get();
    let tbl = precompute();
    for _ in 0 .. q {
        let n: i64 = get();
        let mut ans1 = 0;
        let mut ans2 = 0;
        for (&num_fac, &val) in tbl.iter() {
            if val <= n && ans1 < num_fac {
                ans1 = num_fac;
                ans2 = val;
            }
        }
        println!("{} {}", ans1, ans2);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
