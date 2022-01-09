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

fn dfs(v: usize, to: usize, g: &[Vec<bool>], vis: &mut [bool]) -> Result<(), ()> {
    if v == to {
        return Err(());
    }
    if vis[v] {
        return Ok(());
    }
    vis[v] = true;
    let n = g.len();
    for i in 0..n {
        if g[v][i] {
            dfs(i, to, g, vis)?;
        }
    }
    Ok(())
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        qs: [(i32, usize1, usize1); q],
    }
    let mut g = vec![vec![false; n]; n];
    for (ty, u, v) in qs {
        if ty == 1 {
            g[u][v] ^= true;
            g[v][u] ^= true;
        } else {
            let mut vis = vec![false; n];
            puts!("{}\n", if dfs(u, v, &g, &mut vis).is_err() {
                "Yes"
            } else {
                "No"
            });
        }
    }
}
