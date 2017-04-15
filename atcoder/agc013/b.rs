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

fn dfs(v: usize, vis: &mut [bool], path: &mut Vec<usize>, edges: &[Vec<usize>]) -> bool {
    if vis[v] {
        return false;
    }
    vis[v] = true;
    path.push(v);
    for &w in edges[v].iter() {
        let res = dfs(w, vis, path, edges);
        if res {
            return true;
        }
    }
    true
}

fn solve() {
    let n = get();
    let m = get();
    let mut edges = vec![Vec::new(); n];
    for _ in 0 .. m {
        let u = get::<usize>() - 1;
        let v = get::<usize>() - 1;
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut vis = vec![false; n];
    let mut path1 = Vec::new();
    let mut path2 = Vec::new();
    dfs(0, &mut vis, &mut path1, &edges);
    vis[0] = false;
    dfs(0, &mut vis, &mut path2, &edges);
    path2.reverse();
    path2.pop(); // 0 disappears
    path2.append(&mut path1);
    println!("{}", path2.len());
    for i in 0 .. path2.len() {
        print!("{}{}", path2[i] + 1, if i == path2.len() - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
