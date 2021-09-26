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

fn dfs(v: usize, g: &[Vec<usize>], vis: &mut [bool], c: &[i64], pot: &mut [i64],
       x: i64, basis: &mut Vec<i64>) {
    if vis[v] {
        let mut diff = x ^ pot[v];
        for &b in basis.iter() {
            diff = min(diff, diff ^ b);
        }
        if diff != 0 {
            basis.push(diff);
        }
        return;
    }
    vis[v] = true;
    pot[v] = x;
    for &w in &g[v] {
        dfs(w, g, vis, c, pot, x ^ c[v], basis);
    }
}

// Tags: cycle-basis
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize, q: usize,
        c: [i64; n],
        ab: [(usize1, usize1); m],
        sg: [(usize1, usize1); q],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut vis = vec![false; n];
    let mut basis = vec![];
    let mut pot = vec![0; n];
    dfs(0, &g, &mut vis, &c, &mut pot, 0, &mut basis);
    basis.sort(); basis.reverse();
    // eprintln!("basis = {:?}", basis);
    for (s, g) in sg {
        let mut ans = c[g] ^ pot[s] ^ pot[g];
        // eprintln!("ans = {}", ans);
        for &b in &basis {
            ans = max(ans, ans ^ b);
        }
        puts!("{}\n", ans);
    }
}
