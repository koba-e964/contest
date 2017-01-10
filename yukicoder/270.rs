#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

// Returns the least index of elements that are modified, wrapped with Some.
// If the entire array is reversed, it returns None instead.
// v's elements must be pairwise distinct.
fn next_permutation<T: Ord>(v: &mut [T]) -> Option<usize> {
    let mut tail_dec: usize = 1;
    let n = v.len();
    while tail_dec < n {
        if v[n - tail_dec - 1] > v[n - tail_dec] {
            tail_dec += 1;
        } else {
            break;
        }
    }
    // v[n - tail_dec .. n] is strictly decreasing
    if tail_dec < n {
        let x = n - tail_dec - 1;
        let mut y = n;
        {
            let pivot = &v[x];
            for i in (n - tail_dec .. n).rev() {
                if v[i] > *pivot {
                    y = i;
                    break;
                }
            }
            assert!(y < n);
        }
        v.swap(x, y);
    }
    v[n - tail_dec .. n].reverse();
    if tail_dec < n {
        Some(n - tail_dec - 1)
    } else {
        None
    }
}

fn main() {
    let n: usize = get();
    let k: usize = get();
    let mut p: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let b: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut mods: Vec<Vec<(i64, usize)>> = vec![Vec::new(); n];
    for i in 0 .. n {
        mods[i].push((p[i], 1));
    }
    let mut tot: i64 = 0;
    for stage in 1 .. (k + 1) {
        let ret = match next_permutation(&mut p) {
            Some(idx) => idx,
            None => 0,
        };
        for i in ret .. n {
            mods[i].push((p[i], stage));
        }
    }
    for i in 0 .. n {
        mods[i].push((0x1145141919810893, k + 1));
    }
    for i in 0 .. n {
        let m = &mods[i];
        for j in 0 .. m.len() - 1 {
            tot += (b[i] - m[j].0).abs() * ((m[j + 1].1 - m[j].1) as i64);
        }
    }
    println!("{}", tot);
}
