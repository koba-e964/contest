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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn choose_player(first: bool) {
    if first {
        println!("First");
    } else {
        println!("Second");
    }
}

fn resp(a: &[usize]) {
    for i in 0..a.len() {
        print!("{}{}", a[i] + 1, if i + 1 == a.len() { "\n" } else { " " });
    }
}

fn dfs(v: usize, g: &[Vec<usize>], vis: &mut [bool], marker: &mut [bool]) {
    let n = g.len() / 2;
    if vis[v % n] {
        return;
    }
    vis[v % n] = true;
    marker[v] = true;
    for &w in &g[v] {
        dfs((w + n) % (2 * n), g, vis, marker);
    }
}

fn solve() {
    let n: usize = get();
    if n % 2 == 0 {
        choose_player(true);
        let mut a = vec![0; 2 * n];
        for i in 0..n {
            a[i] = i;
            a[i + n] = i;
        }
        resp(&a);
        let x: i32 = get();
        assert_eq!(x, 0);
        return;
    }
    choose_player(false);
    let mut a = vec![0usize; 2 * n];
    for i in 0..2 * n {
        a[i] = get::<usize>() - 1;
    }
    let mut pairs = vec![vec![]; n];
    for i in 0..2 * n {
        pairs[a[i]].push(i);
    }
    let mut g = vec![vec![]; 2 * n];
    for i in 0..n {
        let a = pairs[i][0];
        let b = pairs[i][1];
        g[a].push(b);
        g[b].push(a);
    }
    let mut vis = vec![false; n];
    let mut marker = vec![false; 2 * n];
    for i in 0..2 * n {
        if !vis[i % n] {
            dfs(i, &g, &mut vis, &mut marker);
        }
    }
    let mut sum = 0;
    for i in 0..2 * n {
        if marker[i] {
            sum = (sum + i) % (2 * n);
        }
    }
    let mut ary = vec![];
    for i in 0..2 * n {
        if marker[i] == (sum != 0) {
            ary.push(i);
        }
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        let idx = a[ary[i]];
        ans[idx] = ary[i];
    }
    assert_eq!(ans.len(), n);
    resp(&ans);
    let x: i32 = get();
    assert_eq!(x, 0);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
