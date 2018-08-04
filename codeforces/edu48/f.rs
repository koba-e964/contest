#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};

const DEBUG: bool = false;

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

fn dfs(v: usize, par: usize, g: &[Vec<(usize, i64)>], in_path: &mut [bool])
       -> (i64, Vec<(usize, i64)>, bool) {
    if v == g.len() - 1 {
        in_path[v] = true;
        return (0, vec![(v, 0)], true);
    }
    for &(w, d) in &g[v] {
        if w == par { continue; }
        let (sub, mut path, is) = dfs(w, v, g, in_path);
        if is {
            in_path[v] = true;
            path.push((v, d));
            return (sub + d, path, true);
        }
    }
    (0, Vec::new(), false)
}
fn dfs2(v: usize, par: usize, g: &[Vec<(usize, i64)>],
        path: &[bool], affix: &mut [i64])
        -> Result<usize, ()> {
    let mut num_vert = 1;
    let mut last_weight = 0;
    for &(w, d) in &g[v] {
        if w == par { continue; }
        if path[w] {
            dfs2(w, v, g, path, affix)?;
            continue;
        }
        num_vert += dfs2(w, v, g, path, affix)?;
        last_weight = d;
    }
    if num_vert >= 3 {
        return Err(());
    }
    if path[v] {
        affix[v] = last_weight;
    }
    Ok(num_vert)
}
const INF: i64 = 1 << 60;

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    let n: usize = get();
    let m = get();
    let mut g = vec![Vec::new(); n];
    for _ in 0 .. n - 1 {
        let v = get::<usize>() - 1;
        let u = get::<usize>() - 1;
        let w: i64 = get();
        g[v].push((u, w));
        g[u].push((v, w));
    }
    let mut in_path = vec![false; n];
    let (orig_dist, mut path, _) = dfs(0, n, &g, &mut in_path);
    path.reverse();
    if DEBUG {
        eprintln!("path = {:?}", path);
    }
    let mut affix = vec![0; n];
    let mut delta = 0;
    let p = path.len();
    if let Ok(_) = dfs2(0, n, &g, &in_path, &mut affix) {
        delta = -INF;
        let mut affix2 = vec![0; p];
        let mut nxt = vec![0; p];
        for i in 0 .. p {
            affix2[i] = affix[path[i].0];
            nxt[i] = path[i].1;
        }
        let mut acc = vec![0; p];
        for i in 0 .. p - 1 {
            acc[i + 1] = acc[i] + nxt[i];
        }
        let affix = affix2;
        if DEBUG {
            eprintln!("affix2 = {:?}", affix);
            eprintln!("nxt = {:?}", nxt);
            eprintln!("acc = {:?}", acc);
        }
        let mut ma2 = -INF; // max_{j < i}(affix2[j] + acc[j])
        let mut lazy = Vec::new();
        for i in 0 .. p {
            delta = max(delta, ma2 + affix[i] - acc[i]);
            for l in lazy.drain(..) {
                if affix[i] > 0 {
                    delta = max(delta, l + affix[i] - acc[i]);
                }
                ma2 = max(ma2, l);
            }
            if affix[i] == 0 {
                lazy.push(affix[i] + acc[i]);
            } else {
                ma2 = max(ma2, affix[i] + acc[i]);
            }
        }
    }
    for _ in 0 .. m {
        let x: i64 = get();
        writeln!(out, "{}", orig_dist + min(0, x + delta)).unwrap();
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
