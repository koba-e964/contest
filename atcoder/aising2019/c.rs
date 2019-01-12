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
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn dfs(v: usize, vis: &mut [bool], g: &[Vec<usize>], col: &[bool])
       -> (i64, i64) {
    if vis[v] { return (0, 0); }
    vis[v] = true;
    let (mut bc, mut wc) = if col[v] { (1, 0) } else { (0, 1) };
    for &w in g[v].iter() {
        let (x, y) = dfs(w, vis, g, col);
        bc += x;
        wc += y;
    }
    (bc, wc)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        h: usize,
        w: usize,
        s: [chars; h],
    }
    let mut g = vec![Vec::new(); h * w];
    for i in 0 .. h - 1 {
        for j in 0 .. w {
            if s[i][j] != s[i + 1][j] {
                let a = i * w + j;
                let b = i * w + w + j;
                g[a].push(b);
                g[b].push(a);
            }
        }
    }
    for i in 0 .. h {
        for j in 0 .. w - 1 {
            if s[i][j] != s[i][j + 1] {
                let a = i * w + j;
                let b = i * w + j + 1;
                g[a].push(b);
                g[b].push(a);
            }
        }
    }
    let mut col = vec![false; h * w];
    for i in 0 .. h {
        for j in 0 .. w {
            col[i * w + j] = s[i][j] == '#';
        }
    }
    let mut vis = vec![false; h * w];
    let mut tot = 0;
    for i in 0 .. h * w {
        if vis[i] { continue; }
        let (b, w) = dfs(i, &mut vis, &g, &col);
        tot += b * w;
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
