use std::cmp::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, g: &[Vec<(usize, usize)>], vis: &mut [bool], tree: &mut [Vec<(usize, usize)>], s: &mut Vec<usize>) {
    if vis[v] { return; }
    vis[v] = true;
    s.push(v);
    for &(w, eidx) in &g[v] {
        if vis[w] { continue; }
        tree[v].push((w, eidx));
        tree[w].push((v, eidx));
        dfs(w, g, vis, tree, s);
    }
}

fn dfs2(v: usize, pareidx: usize, g: &[Vec<(usize, usize)>], col: &mut [bool], picked: &mut Vec<usize>) -> bool {
    let mut ret = col[v];
    for &(w, eidx) in &g[v] {
        if pareidx == eidx { continue; }
        ret ^= dfs2(w, eidx, g, col, picked);
    }
    if ret {
        picked.push(pareidx);
    }
    ret
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize, k: usize,
        ab: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        let (a, b) = ab[i];
        g[a].push((b, i + 1));
        g[b].push((a, i + 1));
    }
    let mut vis = vec![false; n];
    let mut conn = vec![];
    let mut tree = vec![vec![]; n];
    for i in 0..n {
        if !vis[i] {
            let mut s = vec![];
            dfs(i, &g, &mut vis, &mut tree, &mut s);
            conn.push((i, s));
        }
    }
    let mut col = vec![false; n];
    let mut k = k;
    if k % 2 != 0 {
        puts!("No\n");
        return;
    }
    for v in &conn {
        let s = min(k, v.1.len() / 2 * 2);
        k -= s;
        for i in 0..s {
            col[v.1[i]] = true;
        }
    }
    if k > 0 {
        puts!("No\n");
        return;
    }
    puts!("Yes\n");
    let mut es = vec![];
    for (i, _) in conn {
        dfs2(i, n, &tree, &mut col, &mut es);
    }
    puts!("{}\n", es.len());
    putvec!(es);
}
