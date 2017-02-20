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
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
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

fn dfs(v: usize, edges: &[Vec<usize>], t: &[i64], dp: &mut [i64]) {
    let mut stack = Vec::new();
    stack.push((v, false));
    while let Some((v, post)) = stack.pop() {
        if !post {
            stack.push((v, true));
            for &w in edges[v].iter() {
                stack.push((w, false));
            }
        } else {
            let mut tot = t[v];
            for &w in edges[v].iter() {
                tot += dp[w];
            }
            dp[v] = tot;
        }
    }
}

fn dfs_single(v: usize, edges: &[Vec<usize>],
              each: i64,
              dp: &[i64],
              dp_single: &mut [Option<usize>]) {
    let mut stack = Vec::new();
    stack.push((v, false));
    while let Some((v, post)) = stack.pop() {
        if post {
            for &w in edges[v].iter() {
                if let Some(x) = dp_single[w] {
                    dp_single[v] = Some(x);
                }
            }
        } else {
            if dp[v] == each {
                dp_single[v] = Some(v);
            }
            stack.push((v, true));
            for &w in edges[v].iter() {
                stack.push((w, false));
            }
        }
    }
}
fn dfs_two_subtrees(mut v: usize, edges: &[Vec<usize>],
                    dp_single: &[Option<usize>]) -> Option<(usize, usize)> {
    loop {
        // Search result, together with their indices
        let mut desc = Vec::new();
        for &w in edges[v].iter() {
            let res = dp_single[w];
            desc.push((res, w));
        }
        // None < Some(_). All Some(_)s come first :)
        desc.sort();
        desc.reverse();
        if desc.len() >= 2 && desc[1].0.is_some() {
            return Some((desc[0].0.unwrap(), desc[1].0.unwrap()));
        }
        if desc.len() >= 1 && desc[0].0.is_some() {
            v = desc[0].1;
            continue;
        }
        return None;
    }
}
fn dfs_subtree(v: usize, edges: &[Vec<usize>],
               each: i64,
               dp: &[i64],
               dp_single: &[Option<usize>]) -> Option<(usize, usize)> {
    let mut stack = Vec::new();
    stack.push(v);
    while let Some(v) = stack.pop() {
        if dp[v] == 2 * each {
            for &w in edges[v].iter() {
                if let Some(c) = dp_single[w] {
                    return Some((v, c));
                }
            }
        }
        for &w in edges[v].iter() {
            stack.push(w);
        }
    }
    None
}

fn main() {
    let n = get();
    let mut p = vec![0; n];
    let mut edges = vec![Vec::new(); n];
    let mut t = vec![0; n];
    let mut root = n;
    for i in 0 .. n {
        p[i] = get::<usize>();
        t[i] = get::<i64>();
        if p[i] == 0 {
            root = i;
        } else {
            p[i] -= 1;
            edges[p[i]].push(i);
        }
    }
    let mut dp = vec![0; n];
    dfs(root, &edges, &t, &mut dp);
    if dp[root] % 3 != 0 {
        println!("-1");
        return;
    }
    let each = dp[root] / 3;
    let mut dp2 = vec![None; n];
    dfs_single(root, &edges, each, &dp, &mut dp2);
    if let Some((x, y)) = dfs_two_subtrees(root, &edges, &dp2) {
        println!("{} {}", x + 1, y + 1);
        return;
    }
    for &w in edges[root].iter() {
        if dp2[w].is_some() {
            if let Some((x, y)) = dfs_subtree(w, &edges, each, &dp, &dp2) {
                println!("{} {}", x + 1, y + 1);
                return;
            }
        }
    }
    println!("-1");
}
