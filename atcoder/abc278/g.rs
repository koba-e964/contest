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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn remove(state: &mut Vec<(usize, usize)>, x: usize, l: usize) {
    let x = x as usize;
    let mut idx = 0;
    while idx < state.len() {
        if x >= state[idx].0 {
            idx += 1;
        } else {
            break;
        }
    }
    idx -= 1;
    let (old, len) = state[idx];
    state.remove(idx);
    if x + l < old + len {
        state.insert(idx, (x + l, old + len - x - l));
    }
    if old < x {
        state.insert(idx, (old, x - old));
    }
}

fn main() {
    let n: usize = get();
    let l: usize = get();
    let r: usize = get();
    if l < r {
        println!("First");
        let len = if (n + l) % 2 == 0 { l } else { l + 1 };
        let rem = (n - len) / 2;
        println!("{} {}", rem + 1, len);
        loop {
            let x: i32 = get();
            let y: i32 = get();
            if x <= 0 && y <= 0 {
                return;
            }
            let x = x as usize;
            if x > rem + 1 {
                println!("{} {}", x - rem - len, y);
            } else {
                println!("{} {}", x + rem + len, y);
            }
        }
    }
    assert_eq!(l, r);
    let mut dp = vec![vec![]; n + 1];
    for i in 0..n + 1 {
        let mut seen = HashMap::new();
        for j in l..i + 1 {
            let g = dp[j - l].len() ^ dp[i - j].len();
            seen.insert(g, j - l);
        }
        let mut mex = 0;
        while seen.contains_key(&mex) {
            mex += 1;
        }
        for j in 0..mex {
            dp[i].push(seen[&j]);
        }
    }
    let mut state = vec![(1, n)];
    let mut skipping = false;
    if dp[n].is_empty() {
        println!("Second");
        skipping = true;
    } else {
        println!("First");
    }
    loop {
        if !skipping {
            let mut g = 0;
            for &(_, len) in &state {
                g ^= dp[len].len();
            }
            assert_ne!(g, 0);
            let mut idx = state.len();
            for i in 0..state.len() {
                let len = state[i].1;
                if dp[len].len() ^ g < dp[len].len() {
                    idx = i;
                }
            }
            let (base, len) = state[idx];
            let nxt = base + dp[len][dp[len].len() ^ g];
            println!("{} {}", nxt, l);
            remove(&mut state, nxt, l);
        }
        skipping = false;
        let x: i32 = get();
        let y: i32 = get();
        if x <= 0 && y <= 0 {
            return;
        }
        remove(&mut state, x as usize, l);
    }
}
