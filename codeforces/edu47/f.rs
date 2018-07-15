#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write};
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn get_word<R: std::io::Read>(br: &mut std::io::BufReader<R>) -> String {
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = br.read(&mut u8b);
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
fn get<R: std::io::Read, T: std::str::FromStr>(br: &mut std::io::BufReader<R>) -> T { get_word(br).parse().ok().unwrap() }

fn dfs(v: usize, par: Option<usize>, g: &[Vec<(usize)>], ans: &mut [usize]) -> (Vec<i32>, usize) {
    let mut pool = Vec::new();
    for &w in g[v].iter() {
        if Some(w) == par { continue; }
        let sub = dfs(w, Some(v), g, ans);
        pool.push((sub.0.len(), sub));
    }
    if pool.is_empty() {
        ans[v] = 0;
        return (vec![1; 1], 0);
    }
    pool.sort_unstable_by_key(|x| Reverse(x.0));
    let mut princ = std::mem::replace(&mut (pool[0].1).0, Vec::new());
    let princlen = princ.len();
    let midx = (pool[0].1).1;
    let ma = princ[princlen - 1 - midx]; // reversed!
    let mut auxlen = 0;
    let mut newma = ma;
    for i in 1 .. pool.len() {
        let len = pool[i].0;
        auxlen = max(auxlen, len);
        for j in 0 .. len {
            princ[princlen - 1 - j] += (pool[i].1).0[len - 1 - j];
            newma = max(newma, princ[princlen - 1 - j]);
        }
    }
    princ.push(1);
    let mut idx = midx + 1;
    for j in 0 .. auxlen + 1 {
        if newma == princ[princlen - j] {
            idx = j;
            break;
        }
    }
    ans[v] = idx;
    (princ, idx)
}

fn solve() {
    let stdin = std::io::stdin();
    let mut br = std::io::BufReader::new(stdin.lock());
    let n = get(&mut br);
    let mut g = vec![Vec::new(); n];
    for _ in 0 .. n - 1 {
        let x = get::<_, usize>(&mut br) - 1;
        let y = get::<_, usize>(&mut br) - 1;
        g[x].push(y);
        g[y].push(x);
    }
    let mut ans = vec![0; n];
    dfs(0, None, &g, &mut ans);
    // http://keens.github.io/blog/2017/10/05/rustdekousokunahyoujunshutsuryoku/
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    for v in ans {
        writeln!(out, "{}", v).unwrap();
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 2 * 104_857_600; // 200 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
