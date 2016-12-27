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

fn main() {
    let n: usize = get();
    let m: usize = get();
    let w: Vec<usize> = (0 .. n).map(|_| get()).collect();
    let e: Vec<(usize, usize)> = (0 .. m).map(|_| {
        let u: usize = get();
        let v: usize = get();
        (u - 1, v - 1)}).collect();
    let md: i64 = 1_000_000_007;
    let mut tot: i64 = 0;
    // precompute the number of surjections, for all w_1, w_2
    const W: usize = 1001;
    let mut surj: Vec<Vec<i64>> = vec![vec![0; W]; W];
    surj[0][0] = 1;
    for i in 1 .. W {
        for j in 1 .. W {
            let tmp = (j as i64) * (surj[i - 1][j] + surj[i - 1][j - 1]);
            surj[i][j] = tmp % md;
        }
    }
    for i in 0 .. n {
        // collect all j s.t. w[j] >= w[i]
        let mut edges: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(u, v) in &e {
            if w[u] >= w[i] && w[v] >= w[i] {
                edges[v].push(u);
            }
        }
        let mut reach: Vec<bool> = vec![false; n];
        let mut que: VecDeque<usize> = VecDeque::new();
        que.push_back(i);
        while let Some(v) = que.pop_front() {
            if reach[v] { continue; }
            reach[v] = true;
            for &w in &edges[v] {
                que.push_back(w);
            }
        }
        for j in 0 .. n {
            if reach[j] {
                tot = (tot + surj[w[j]][w[i]]) % md;
            }
        }
    }
    println!("{}", tot);
}
