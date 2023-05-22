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

fn main() {
    let n: usize = get();
    let q: usize = get();
    let mut g = vec![BTreeSet::new(); n];
    let mut ans = n;
    for _ in 0..q {
        let ty: i32 = get();
        let u = get::<usize>() - 1;
        if ty == 1 {
            let v = get::<usize>() - 1;
            if g[u].is_empty() {
                ans -= 1;
            }
            g[u].insert(v);
            if g[v].is_empty() {
                ans -= 1;
            }
            g[v].insert(u);
        } else {
            let mut res = vec![];
            for &w in &g[u] {
                res.push(w);
            }
            if !g[u].is_empty() {
                ans += 1;
            }
            g[u].clear();
            for &w in &res {
                g[w].remove(&u);
                if g[w].is_empty() {
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}
