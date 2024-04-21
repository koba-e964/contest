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

fn dfs(v: usize, g: &[Vec<(usize, i64, i64)>], x: i64, y: i64, coord: &mut [(i64, i64)], vis: &mut [bool]) {
    if vis[v] {
        return;
    }
    vis[v] = true;
    coord[v] = (x, y);
    for &(w, dx, dy) in &g[v] {
        dfs(w, g, x + dx, y + dy, coord, vis);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        abxy: [(usize1, usize1, i64, i64); m],
    }
    let mut g = vec![vec![]; n];
    for (a, b, x, y) in abxy {
        g[a].push((b, x, y));
        g[b].push((a, -x, -y));
    }
    let mut vis = vec![false; n];
    let mut coord = vec![(0, 0); n];
    dfs(0, &g, 0, 0, &mut coord, &mut vis);
    for i in 0..n {
        if vis[i] {
            puts!("{} {}\n", coord[i].0, coord[i].1);
        } else {
            puts!("undecidable\n");
        }
    }
}
