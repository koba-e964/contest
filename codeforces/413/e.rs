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

const INF: i64 = 1 << 50;


fn calc(kind: &[Vec<i64>], m: usize, k: usize) -> i64 {
    let mut it = [0; 4];
    let mut res = INF;;
    let mut cur_cost = 0;
    for i in 0 .. kind[3].len() + 1 {
        while it[3] < i {
            cur_cost += kind[3][it[3]]; it[3] += 1;
            for u in 0 .. 3 {
                if it[u] > 0 {
                    it[u] -= 1;
                    cur_cost -= kind[u][it[u]];
                }
            }
        }
        for u in 1 .. 3 {
            while it[u] + it[3] < k && it[u] < kind[u].len() {
                cur_cost += kind[u][it[u]];
                it[u] += 1;
            }
        }

        while it[0] + it[1] + it[2] + it[3] < m {
            let mut mini = 4;
            let mut mi = INF;
            for u in 0 .. 3 {
                if it[u] < kind[u].len() && kind[u][it[u]] < mi {
                    mi = kind[u][it[u]];
                    mini = u;
                }
            }
            if mi == INF { break; }
            cur_cost += kind[mini][it[mini]];
            it[mini] += 1;
        }
        if it[0] + it[1] + it[2] + it[3] == m &&
            it[1] + it[3] >= k &&
            it[2] + it[3] >= k {
                res = min(res, cur_cost);
            }
    }
    res
}


/// Reference: http://codeforces.com/contest/799/submission/27046407
fn solve() {
    let n: usize = get();
    let m: usize = get();
    let k: usize = get();
    let c: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let a: usize = get();
    let x: Vec<usize> = (0 .. a).map(|_| get()).collect();
    let b: usize = get();
    let y: Vec<usize> = (0 .. b).map(|_| get()).collect();
    let mut kind: Vec<usize> = vec![0; n];
    for v in x {
        kind[v - 1] |= 1;
    }
    for v in y {
        kind[v - 1] |= 2;
    }
    let mut pool = vec![Vec::new(); 4];
    for i in 0 .. n {
        pool[kind[i]].push(c[i]);
    }
    for i in 0 .. 4 {
        pool[i].sort();
    }
    let res = calc(&pool, m, k);
    if res == INF {
        println!("-1");
    } else {
        println!("{}", res);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
