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

const INF: i64 = 1i64 << 50;

fn dfs(v: usize, p: Option<usize>, a: &[i64], dp: &mut [i64], edges: &[Vec<usize>]) {
    let mut inf = false;
    for &w in edges[v].iter() {
        if Some(w) == p {
            continue;
        }
        dfs(w, Some(v), a, dp, edges);
        if dp[w] < a[v] {
            inf = true;
        }
    }
    dp[v] = if inf { INF } else { a[v] };
}

fn solve(v: usize, a: &[i64], edges: &[Vec<usize>]) -> bool {
    let n = edges.len();
    let mut dp = vec![0; n];
    dfs(v, None, a, &mut dp, edges);
    dp[v] == INF
}

// I implemented this code after I read the editorial.
fn main() {
    let n = get();
    let mut edges = vec![Vec::new(); n];
    let a: Vec<_> = (0 .. n).map(|_| get()).collect();
    for _ in 0 .. n - 1 {
        let a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        edges[a].push(b);
        edges[b].push(a);
    }
    let mut ans = Vec::new();
    for i in 0 .. n {
        if solve(i, &a, &edges) {
            ans.push(i)
        }
    }
    for i in 0 .. ans.len() {
        print!("{}{}", ans[i] + 1, if i == ans.len() - 1 { "" } else { " " });
    }
    println!("");
}
