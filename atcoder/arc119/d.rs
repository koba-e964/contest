#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn dfs(v: usize, h: usize, vis: &mut [bool], g: &[Vec<usize>], fst: &mut Vec<usize>, snd: &mut Vec<usize>) {
    if vis[v] {
        return;
    }
    vis[v] = true;
    if v >= h {
        snd.push(v);
    } else {
        fst.push(v);
    }
    for &w in &g[v] {
        dfs(w, h, vis, g, fst, snd);
    }
}

fn dfs2(v: usize, h: usize, vis: &mut [bool], g: &[Vec<usize>], ans: &mut Vec<(usize, usize, char)>) {
    if vis[v] {
        return;
    }
    vis[v] = true;
    for &w in &g[v] {
        let visw = vis[w];
        dfs2(w, h, vis, g, ans);
        if !visw {
            if v >= h {
                ans.push((w, v - h, 'X'));
            } else {
                ans.push((v, w - h, 'Y'));
            }
        }
    }
}

// Tags: construction, dfs
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        h: usize, w: usize,
        s: [chars; h],
    }
    let mut g = vec![vec![]; h + w];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'R' {
                g[i].push(h + j);
                g[h + j].push(i);
            }
        }
    }
    let mut vis = vec![false; h + w];
    let mut a = 0;
    let mut b = 0;
    let mut fixed = vec![];
    let mut volat = vec![];
    let mut cons = vec![];
    for i in 0..h + w {
        if !vis[i] {
            let mut fst = vec![];
            let mut snd = vec![];
            dfs(i, h, &mut vis, &g, &mut fst, &mut snd);
            let idx = cons.len();
            if fst.len() * snd.len() > 0 {
                volat.push(idx);
            } else {
                fixed.push(idx);
                if fst.is_empty() {
                    b += 1;
                } else {
                    a += 1;
                }
            }
            cons.push((fst, snd));
        }
    }
    let vol = volat.len();
    let mut mi = (a * (b + vol), 0);
    for i in 0..vol + 1 {
        mi.chmin(((a + i) * (b + vol - i), i));
    }
    let nvol = mi.1;
    let mut ans = vec![];
    for x in &mut vis {
        *x = false;
    }
    for i in 0..a + b {
        let v = &cons[fixed[i]];
        dfs2(if v.0.is_empty() { v.1[0] } else { v.0[0] },
             h, &mut vis, &g, &mut ans);
    }
    for i in 0..nvol {
        dfs2(cons[volat[i]].0[0], h, &mut vis, &g, &mut ans);
    }
    for i in nvol..volat.len() {
        dfs2(cons[volat[i]].1[0], h, &mut vis, &g, &mut ans);
    }
    puts!("{}\n", ans.len());
    for i in 0..ans.len() {
        let (x, y, k) = ans[i];
        puts!("{} {} {}\n", k, x + 1, y + 1);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
