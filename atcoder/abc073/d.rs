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

/**
 * Returns the least index of elements that are modified, wrapped with Some.
 * If the entire array is reversed, it returns None instead.
 * v's elements must be pairwise distinct.
 */
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


fn solve() {
    let n: usize = get();
    let m = get();
    let rr = get();
    let r: Vec<usize> = (0 .. rr).map(|_| get::<usize>() - 1).collect();
    let mut dist = vec![vec![1 << 29; n]; n];
    for _ in 0 .. m {
        let a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        let c = get();
        dist[a][b] = min(dist[a][b], c);
        dist[b][a] = min(dist[b][a], c);
    }
    for k in 0 .. n {
        for i in 0 .. n {
            for j in 0 .. n {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }
    let mut perm = vec![0; rr];
    for i in 0 .. rr {
        perm[i] = r[i];
    }
    perm.sort();
    let mut mi = 1 << 29;
    loop {
        let mut tot = 0;
        for i in 0 .. rr - 1 {
            tot += dist[perm[i]][perm[i + 1]];
        }
        mi = min(mi, tot);
        if next_permutation(&mut perm).is_none() {
            break;
        }
    }
    println!("{}", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
