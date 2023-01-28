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

fn dfs(v: usize, g: &[Vec<(usize, i64)>], vis: &mut [bool],
    r: usize, root: &mut [usize],
    p: i64, pot: &mut [i64]) -> bool {
    if vis[v] {
        if pot[v] != p {
            return false;
        }
        return true;
    }
    vis[v] = true;
    root[v] = r;
    pot[v] = p;
    let mut ok = true;
    for &(w, c) in &g[v] {
        ok &= dfs(w, g, vis, r, root, p + c, pot);
    }
    ok
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize, q: usize,
        abc: [(usize1, usize1, i64); m],
        xy: [(usize1, usize1); q],
    }
    let mut g = vec![vec![]; n];
    for (a, b, c) in abc {
        g[a].push((b, c));
        g[b].push((a, -c));
    }
    let mut vis = vec![false; n];
    let mut root = vec![0; n];
    let mut pot = vec![0; n];
    let mut ok = vec![false; n];
    for i in 0..n {
        if !vis[i] {
            ok[i] = dfs(i, &g, &mut vis, i, &mut root, 0, &mut pot);
        }
    }
    for (x, y) in xy {
        if root[x] != root[y] {
            puts!("nan\n");
            continue;
        }
        if !ok[root[x]] {
            puts!("inf\n");
            continue;
        }
        puts!("{}\n", pot[y] - pot[x]);
    }
}
